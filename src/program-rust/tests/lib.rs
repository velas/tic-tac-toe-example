use borsh::BorshDeserialize;
use helloworld::{process_instruction, GameState};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use std::mem;

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::new_unique();
    
    let player_one = Pubkey::new_unique();
    let player_two = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "tic-tac-toe", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    program_test.add_account(
        player_one,
        Account {
            lamports: 50000,
            data: vec![0_u8; mem::size_of::<GameState>()],
            owner: program_id,
            ..Account::default()
        },
    );

    program_test.add_account(
        player_two,
        Account {
            lamports: 50000,
            data: vec![0_u8; mem::size_of::<GameState>()],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Verify account has zero greetings
    let tic_tac_account = banks_client
        .get_account(player_one)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    
    assert_eq!(
        GameState::try_from_slice(&tic_tac_account.data)
            .unwrap()
            .play_field[0],
        0
    );

    // Greet once
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1u8, 0, 0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(player_one, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has one greeting
    let greeted_account = banks_client
        .get_account(player_one)
        .await
        .expect("get_account")
        .expect("greeted_account not found");

    assert_eq!(
        GameState::try_from_slice(&greeted_account.data)
            .unwrap()
            .play_field[0],
        1
    );

    // Greet again
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1u8, 0, 1], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(player_two, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has two greetings
    let greeted_account = banks_client
        .get_account(player_one)
        .await
        .expect("get_account")
        .expect("greeted_account not found");
    assert_eq!(
        GameState::try_from_slice(&greeted_account.data)
            .unwrap()
            .play_field[1],
        2
    );
}
