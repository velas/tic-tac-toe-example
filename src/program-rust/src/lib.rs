#![allow(dead_code)]

use borsh::{BorshDeserialize, BorshSerialize};
use agsol_borsh_schema::BorshSchemaTS;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

static WIN_CONDITIONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6]
];

#[derive(BorshSchemaTS, BorshSerialize, BorshDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameCell { Empty, Tic, Tac }

impl Default for GameCell {
    fn default() -> Self { GameCell::Empty }
}

#[derive(BorshSchemaTS, BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameStatus { Uninitialized, PlayerOneTurn, PlayerTwoTurn, GameEnd }

#[derive(BorshSchemaTS, BorshSerialize, BorshDeserialize, Debug)]
pub enum GameInstruction {
    GameReset {
        player_one: Pubkey,
        player_two: Pubkey
    },
    MakeTurn {
        row: u8,
        col: u8
    }
}

/// Define the type of state stored in game account
#[derive(BorshSchemaTS, BorshSerialize, BorshDeserialize, Debug)]
pub struct GameState {
    pub play_field: [GameCell; 9],
    pub status: GameStatus,
    pub player_one: Pubkey,
    pub player_two: Pubkey
}

impl GameState {
    pub fn pretty_print(&self) -> String {
        let mut ascii_draw = String::new();
        for row in 0u8..3 {
            for col in 0u8..3 {
                match self.play_field[(row * 3 + col) as usize] {
                    GameCell::Tic => ascii_draw.push('X'),
                    GameCell::Tac => ascii_draw.push('0'),
                    GameCell::Empty => ascii_draw.push('.')
                }
            }
            ascii_draw.push('\n');
        }
        ascii_draw
    }
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // Accounts of the game and player
    instruction_data: &[u8], // Serialized `GameInstruction`
) -> ProgramResult {
    msg!("Tic-tac-toe game program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let game_account = next_account_info(accounts_iter)?;

    let player_account = next_account_info(accounts_iter)?;
    if !player_account.is_signer {
        msg!("Player is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // The account must be owned by the program in order to modify its data
    if game_account.owner != program_id {
        msg!("Game account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut game_state = GameState::try_from_slice(&game_account.data.borrow())?;

    // Deserializing instruction into typed variable using `borsh` lib
    let instruction = GameInstruction::try_from_slice(instruction_data)?;

    // Applying tic-tac-toe game logic here or reset the game
    apply_instruction(&mut game_state, instruction, player_account.key)?;

    // Check if game has ended
    check_game_end(&mut game_state)?;

    // Save updated state
    game_state.serialize(&mut &mut game_account.data.borrow_mut()[..])?;

    Ok(())
}

fn apply_instruction(
    game: &mut GameState,
    instruction: GameInstruction,
    player_key: &Pubkey
) -> ProgramResult
{
    match instruction {
        GameInstruction::GameReset { player_one, player_two } => {
            // TODO: or if both players are signers
            if game.status == GameStatus::GameEnd || game.status == GameStatus::Uninitialized {
                game.player_one = player_one;
                game.player_two = player_two;
                game.play_field = [GameCell::Empty; 9];
                game.status = GameStatus::PlayerOneTurn;
            } else {
                msg!("You can't reset an in-progress game");
                return Err(ProgramError::InvalidInstructionData);
            }
        },
        GameInstruction::MakeTurn { row, col } => {
            if row <= 3 && col <= 3 {
                let idx = (row * 3 + col) as usize;
                match (game.status, game.play_field[idx]) {
                    (GameStatus::PlayerOneTurn, GameCell::Empty) => {
                        if player_key == &game.player_one {
                            game.play_field[idx] = GameCell::Tic;
                            game.status = GameStatus::PlayerTwoTurn;
                        } else {
                            msg!("You are not a player of this game");
                            return Err(ProgramError::InvalidInstructionData);
                        }
                    },
                    (GameStatus::PlayerTwoTurn, GameCell::Empty) => {
                        if player_key == &game.player_two {
                            game.play_field[idx] = GameCell::Tac;
                            game.status = GameStatus::PlayerOneTurn;
                        } else {
                            msg!("You are not a player of this game");
                            return Err(ProgramError::InvalidInstructionData);
                        }
                    },
                    (GameStatus::PlayerOneTurn, _) | (GameStatus::PlayerTwoTurn, _) => {
                        msg!("This cell is not empty");
                        return Err(ProgramError::InvalidInstructionData);
                    },
                    _ => {
                        msg!("Making turn is not allowed, initialize game first");
                        return Err(ProgramError::InvalidInstructionData);
                    }
                }
            }
        }
    }

    Ok(())
}

fn check_game_end(game: &mut GameState) -> ProgramResult {
    for coord in WIN_CONDITIONS.iter() {
        let x = game.play_field[coord[0]];
        let y = game.play_field[coord[1]];
        let z = game.play_field[coord[2]];

        if x == y && y == z && x != GameCell::Empty {
            game.status = GameStatus::GameEnd;
        }
    }

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<GameState>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let player = account.clone();

        let accounts = vec![account, player];

        let game_reset = GameInstruction::GameReset {
            player_one: Pubkey::default(),
            player_two: Pubkey::default()
        }.try_to_vec().unwrap();

        let make_turn_1 = GameInstruction::MakeTurn { col: 0, row: 0}.try_to_vec().unwrap();
        let make_turn_2 = GameInstruction::MakeTurn { col: 1, row: 1}.try_to_vec().unwrap();

        assert_eq!(make_turn_1, [0x01, 0x00, 0x00]);
        assert_eq!(make_turn_2, [0x01, 0x01, 0x01]);

        process_instruction(&program_id, &accounts, &game_reset).unwrap();
        let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        println!("{}", &account.pretty_print());

        process_instruction(&program_id, &accounts, &make_turn_1).unwrap();
        let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        println!("{}", &account.pretty_print());

        process_instruction(&program_id, &accounts, &make_turn_2).unwrap();
        let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        println!("{}", &account.pretty_print());
    }
}
