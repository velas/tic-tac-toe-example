use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    // borsh::get_packed_len as borsh_packed_len,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameCell { Empty, Tic, Tac }

impl Default for GameCell {
    fn default() -> Self { GameCell::Empty }
}

#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq)]
pub enum Turn { PlayerOne, PlayerTwo }

#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Debug)]
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

/// Define the type of state stored in accounts
#[derive(BorshSchema, BorshSerialize, BorshDeserialize, Debug)]
pub struct GameState {
    pub play_field: [GameCell; 9],
    pub next_turn: Turn,
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
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let game_account = next_account_info(accounts_iter)?;
    let player_account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if game_account.owner != program_id {
        msg!("Game account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut game_state = GameState::try_from_slice(&game_account.data.borrow())?;
    
    let instruction = GameInstruction::try_from_slice(instruction_data)?;

    match instruction {
        GameInstruction::GameReset { player_one, player_two } => {
            game_state.player_one = player_one;
            game_state.player_two = player_two;
            game_state.play_field = [GameCell::Empty; 9];
        },
        GameInstruction::MakeTurn { row, col } => {
            if row <= 3 && col <= 3 {
                let idx = (row * 3 + col) as usize;
                if game_state.next_turn == Turn::PlayerOne {
                    game_state.play_field[idx] = GameCell::Tic;
                    game_state.next_turn = Turn::PlayerTwo;
                } else {
                    game_state.play_field[idx] = GameCell::Tac;
                    game_state.next_turn = Turn::PlayerOne;
                }
            }
        }
    }
    
    game_state.serialize(&mut &mut game_account.data.borrow_mut()[..])?;

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
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let player = account.clone();

        let accounts = vec![account, player];

        let make_turn_1 = GameInstruction::MakeTurn { col: 0, row: 0}.try_to_vec().unwrap();
        let make_turn_2 = GameInstruction::MakeTurn { col: 1, row: 1}.try_to_vec().unwrap();
        // let game_reset = GameInstruction::GameReset {}.try_to_vec().unwrap();

        assert_eq!(make_turn_1, [0x01, 0x00, 0x00]);
        assert_eq!(make_turn_2, [0x01, 0x01, 0x01]);
        // assert_eq!(game_reset, [0x00]);

        process_instruction(&program_id, &accounts, &make_turn_1).unwrap();
        let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        println!("{}", &account.pretty_print());

        process_instruction(&program_id, &accounts, &make_turn_2).unwrap();
        let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        println!("{}", &account.pretty_print());

        // process_instruction(&program_id, &accounts, &game_reset).unwrap();
        // let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        // println!("{}", &account.pretty_print());
    }
}
