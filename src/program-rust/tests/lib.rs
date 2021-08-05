use borsh::BorshDeserialize;
use helloworld::{process_instruction, GameState};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Signer, Keypair},
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
            data: vec![0_u8; 10], // fixme: wrong size
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

    // Game field is empty
    let tic_tac_account = banks_client
        .get_account(game_account_pubkey)
        .await
        .expect("get_account")
        .expect("game_account not found");

    assert_eq!(
        GameState::try_from_slice(&tic_tac_account.data).unwrap().play_field,
        [0, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    // Make move
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
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
        [1, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    // Make second turn
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1u8, 0, 1], // ignored but makes the instruction unique in the slot
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
        [1, 2, 0, 0, 0, 0, 0, 0, 0]
    );
}
