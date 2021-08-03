use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq)]
pub enum Sign { Tic, Tac }

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    GameReset,
    MakeTurn {
        row: u8,
        col: u8
    }
}

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameState {
    pub play_field: [u8; 9],
    pub next_turn: Sign
}

impl GameState {
    pub fn pretty_print(&self) -> String {
        let mut ascii_draw = String::new();
        for row in 0u8..3 {
            for col in 0u8..3 {
                match self.play_field[(row * 3 + col) as usize] {
                    1 => ascii_draw.push('X'),
                    2 => ascii_draw.push('0'),
                    _ => ascii_draw.push('.')
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
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut game_account = GameState::try_from_slice(&account.data.borrow())?;
    
    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Instruction::GameReset => {
            game_account.play_field = [0; 9];
        },
        Instruction::MakeTurn { row, col } => {
            let idx = (row * 3 + col) as usize;
            if idx <= 8 {
                if game_account.next_turn == Sign::Tic {
                    game_account.play_field[idx] = 1;
                    game_account.next_turn = Sign::Tac;
                } else {
                    game_account.play_field[idx] = 2;
                    game_account.next_turn = Sign::Tic;
                }
            }
        }
    }
    
    game_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

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

        let accounts = vec![account];

        let make_turn_1 = Instruction::MakeTurn { col: 0, row: 0}.try_to_vec().unwrap();
        let make_turn_2 = Instruction::MakeTurn { col: 1, row: 1}.try_to_vec().unwrap();
        let game_reset = Instruction::GameReset.try_to_vec().unwrap();

        process_instruction(&program_id, &accounts, &make_turn_1).unwrap();
        // let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        println!("{}", &account.pretty_print());

        process_instruction(&program_id, &accounts, &make_turn_2).unwrap();
        let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        println!("{}", &account.pretty_print());

        process_instruction(&program_id, &accounts, &game_reset).unwrap();
        let account = GameState::try_from_slice(&accounts[0].data.borrow()).unwrap();
        println!("{}", &account.pretty_print());
    }

    #[test]
    fn what_is_borsh() {
        {
            #[derive(BorshSerialize, BorshDeserialize, Debug)]
            enum SimpleEnum {
                A,
                B
            }

            assert_eq!(SimpleEnum::A.try_to_vec().unwrap(), [0]);
            assert_eq!(SimpleEnum::B.try_to_vec().unwrap(), [1]);
        }

        {
            #[derive(BorshSerialize, BorshDeserialize, Debug)]
            enum DiscriminatedUnion {
                A,
                B(u32),
                C { foo: u8, bar: u16 }
            }
            
            assert_eq!(DiscriminatedUnion::A.try_to_vec().unwrap(), [0]);
            assert_eq!(DiscriminatedUnion::B(42).try_to_vec().unwrap(), [1, 42, 0, 0, 0]);
            assert_eq!(DiscriminatedUnion::B(42 << 8).try_to_vec().unwrap(), [1, 0, 42, 0, 0]);
            assert_eq!(DiscriminatedUnion::C { foo: 7, bar: 18 }.try_to_vec().unwrap(), [2, 7, 18, 0]);
        }

        {
            #[derive(BorshSerialize, BorshDeserialize, Debug)]
            enum DynSizedEnum {
                A,
                B(Vec<u16>)
            }

            let dyn_sized = DynSizedEnum::B(vec![11, 22, 33]).try_to_vec().unwrap();
            assert_eq!(dyn_sized, [1, 3, 0, 0, 0, 11, 0, 22, 0, 33, 0]);
        }
    }
}
