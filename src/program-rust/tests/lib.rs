use borsh::{BorshSerialize, BorshDeserialize};
use helloworld::{GameCell, GameInstruction, GameState, process_instruction};
use solana_program::borsh::get_packed_len as borsh_packed_len;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::new_unique();
    
    let game_account_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "tic-tac-toe", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    program_test.add_account(
        game_account_pubkey,
        Account {
            lamports: 50000,
            data: vec![0_u8; borsh_packed_len::<GameState>()],
            owner: program_id,
            ..Account::default()
        },
    );

    let player_one = Keypair::new();
    let player_two = Keypair::new();

    for player in [&player_one, &player_two] {
        program_test.add_account(
            player.pubkey(),
            Account {
                lamports: 50000,
                ..Account::default()
            },
        );
    }

    let (mut banks_client, _, recent_blockhash) = program_test.start().await;

    // Play field is empty
    let tic_tac_account = banks_client
        .get_account(game_account_pubkey)
        .await
        .expect("get_account")
        .expect("game_account not found");

    assert_eq!(
        GameState::try_from_slice(&tic_tac_account.data).unwrap().play_field,
        [
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
        ]
    );

    // Make turn
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            // &GameInstruction::MakeTurn { row: 0, col: 0 }.try_to_vec().unwrap(),
            &[1u8, 0, 0],
            vec![
                AccountMeta::new(game_account_pubkey, false),
                AccountMeta::new(player_one.pubkey(), true)
            ],
        )],
        Some(&player_one.pubkey()),
    );
    transaction.sign(&[&player_one], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify first player made his turn
    let game_account = banks_client
        .get_account(game_account_pubkey)
        .await
        .expect("get_account")
        .expect("game_account not found");

    assert_eq!(
        GameState::try_from_slice(&game_account.data).unwrap().play_field,
        [
            GameCell::Tic, GameCell::Empty, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
        ]
    );

    // Make second turn
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            // &GameInstruction::MakeTurn { row: 0, col: 1 }.try_to_vec().unwrap(),
            &[1u8, 0, 1],
            vec![
                AccountMeta::new(game_account_pubkey, false),
                AccountMeta::new(player_two.pubkey(), true),
            ],
        )],
        Some(&player_two.pubkey()),
    );
    transaction.sign(&[&player_two], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify second player made his turn
    let game_account = banks_client
        .get_account(game_account_pubkey)
        .await
        .expect("get_account")
        .expect("game_account not found");

    assert_eq!(
        GameState::try_from_slice(&game_account.data).unwrap().play_field,
        [
            GameCell::Tic, GameCell::Tac, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
        ]
    );
}

#[test]
fn lol() {
    let a: &[u8] = &GameInstruction::MakeTurn { row: 0, col: 0 }.try_to_vec().unwrap();
    assert_eq!(a, [1u8, 0, 0]);
}
