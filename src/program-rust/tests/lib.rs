use borsh::BorshDeserialize;
use tic_tac_toe::{GameCell, GameInstruction, GameState, GameStatus, process_instruction};
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
async fn test_tic_tac_toe() {
    let program_id = Pubkey::new_unique();

    let game_account_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "tic_tac_toe", // Run the BPF version with `cargo test-bpf`
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
    let not_a_player = Keypair::new();

    for player in [&player_one, &player_two, &not_a_player] {
        program_test.add_account(
            player.pubkey(),
            Account {
                lamports: 50000,
                ..Account::default()
            },
        );
    }

    let (mut banks_client, _, recent_blockhash) = program_test.start().await;

    let game_state = get_game_state(&mut banks_client, game_account_pubkey).await;
    assert_eq!(game_state.status, GameStatus::Uninitialized);

    // Initialize game
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &GameInstruction::GameReset {
                player_one: player_one.pubkey(),
                player_two: player_two.pubkey()
            },
            vec![
                AccountMeta::new(game_account_pubkey, false),
                AccountMeta::new(player_one.pubkey(), true)
            ],
        )],
        Some(&player_one.pubkey()),
    );
    transaction.sign(&[&player_one], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let game_state = get_game_state(&mut banks_client, game_account_pubkey).await;
    assert_eq!(game_state.play_field, [GameCell::Empty; 9]);
    assert_eq!(game_state.status, GameStatus::PlayerOneTurn);

    // Make turn
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &GameInstruction::MakeTurn { row: 0, col: 0 },
            vec![
                AccountMeta::new(game_account_pubkey, false),
                AccountMeta::new(player_one.pubkey(), true)
            ],
        )],
        Some(&player_one.pubkey()),
    );
    transaction.sign(&[&player_one], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let game_state = get_game_state(&mut banks_client, game_account_pubkey).await;
    assert_eq!(
        game_state.play_field,
        [
            GameCell::Tic, GameCell::Empty, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
        ]
    );
    assert_eq!(game_state.status, GameStatus::PlayerTwoTurn);

    // Make second turn
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &GameInstruction::MakeTurn { row: 0, col: 1 },
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
    let game_state = get_game_state(&mut banks_client, game_account_pubkey).await;
    assert_eq!(
        game_state.play_field,
        [
            GameCell::Tic, GameCell::Tac, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
            GameCell::Empty, GameCell::Empty, GameCell::Empty,
        ]
    );
    assert_eq!(game_state.status, GameStatus::PlayerOneTurn);

    // Trying to do unauthorized turn
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &GameInstruction::MakeTurn { row: 1, col: 1 },
            vec![
                AccountMeta::new(game_account_pubkey, false),
                AccountMeta::new(not_a_player.pubkey(), true),
            ],
        )],
        Some(&not_a_player.pubkey()),
    );
    transaction.sign(&[&not_a_player], recent_blockhash);
    let res_err = banks_client.process_transaction(transaction).await;
    assert!(res_err.is_err())
}

async fn get_game_state(client: &mut BanksClient, game_pubkey: Pubkey) -> GameState {
    let tic_tac_account = client
        .get_account(game_pubkey)
        .await
        .expect("get_account")
        .expect("game_account not found");

    GameState::try_from_slice(&tic_tac_account.data).unwrap()
}
