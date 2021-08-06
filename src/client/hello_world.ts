/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';

import {
  getPayer,
  getRpcUrl,
  newAccountWithLamports,
  createKeypairFromFile,
} from './utils';

/**
 * Connection to the network
 */
let connection: Connection;

/**
 * Keypair associated to the fees' payer
 */
let payer: Keypair;

/**
 * Hello world's program id
 */
let programId: PublicKey;

/**
 * The public key of the account we are saying hello to
 */
let gamePubkey: PublicKey;

/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-c`
 *   - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'helloworld.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/helloworld.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'helloworld-keypair.json');

/**
 * The state of a game account managed by the tic-tac-toe program
 */
class GameState {
  play_field = [0, 0, 0, 0, 0, 0, 0, 0, 0];
  status = 0;
  player_one = PublicKey.default;
  player_two = PublicKey.default;

  constructor(
    fields: {
      play_field: number[],
      status: number,
      player_one: PublicKey,
      player_two: PublicKey
    } | undefined = undefined
  ) {
    if (fields) {
      this.play_field = fields.play_field;
      this.status = fields.status;
      this.player_one = fields.player_one;
      this.player_two = fields.player_two;
    }
  }
}

/**
 * Borsh schema definition for game account
 */
const GameSchema = new Map([
  [
    GameState, 
    {
      kind: 'struct',
      fields: [
        ['play_field', [9]],
        ['status', 'u8'],
        ['player_one', [32]],
        ['player_two', [32]]
      ]
    }
  ],
]);

/**
 * The expected size of game account.
 */
const GAME_STATE_SIZE = borsh.serialize(
  GameSchema,
  new GameState(),
).length;

/**
 * Establish a connection to the cluster
 */
export async function establishConnection(): Promise<void> {
  const rpcUrl = await getRpcUrl();
  connection = new Connection(rpcUrl, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', rpcUrl, version);
}

/**
 * Establish an account to pay for everything
 */
export async function establishPayer(): Promise<void> {
  let fees = 0;
  if (!payer) {
    const {feeCalculator} = await connection.getRecentBlockhash();

    // Calculate the cost to fund the game account
    fees += await connection.getMinimumBalanceForRentExemption(GAME_STATE_SIZE);

    // Calculate the cost of sending transactions
    fees += feeCalculator.lamportsPerSignature * 100; // wag

    try {
      // Get payer from cli config
      payer = await getPayer();
    } catch (err) {
      // Fund a new payer via airdrop
      payer = await newAccountWithLamports(connection, fees);
    }
  }

  const lamports = await connection.getBalance(payer.publicKey);
  if (lamports < fees) {
    // This should only happen when using cli config keypair
    const sig = await connection.requestAirdrop(
      payer.publicKey,
      fees - lamports,
    );
    await connection.confirmTransaction(sig);
  }

  console.log(
    'Using account',
    payer.publicKey.toBase58(),
    'containing',
    lamports / LAMPORTS_PER_SOL,
    'SOL to pay for fees',
  );
}

/**
 * Check if the hello world BPF program has been deployed
 */
export async function checkProgram(): Promise<void> {
  // Read program id from keypair file
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `solana program deploy dist/program/helloworld.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);

  // Derive the address (public key) of a game account from the program so that it's easy to find later.
  const GAME_SEED = 'hello';
  gamePubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    GAME_SEED,
    programId,
  );

  // Check if the game account has already been created
  const gameAccount = await connection.getAccountInfo(gamePubkey);
  if (gameAccount === null) {
    console.log(
      'Creating account',
      gamePubkey.toBase58(),
      'to play at',
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      GAME_STATE_SIZE,
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: payer.publicKey,
        seed: GAME_SEED,
        newAccountPubkey: gamePubkey,
        lamports,
        space: GAME_STATE_SIZE,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
}

export async function makeTurn(instruction_data: number[]): Promise<void> {
  console.log('Making turn...', gamePubkey.toBase58());
  const instruction = new TransactionInstruction({
    keys: [{pubkey: gamePubkey, isSigner: false, isWritable: true}],
    programId,
    data: Buffer.from(instruction_data),
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}

/**
 * Show status of the game field
 */
export async function reportGame(): Promise<void> {
  const accountInfo = await connection.getAccountInfo(gamePubkey);
  if (accountInfo === null) {
    throw 'Error: cannot find the game account';
  }

  const gameState: GameState = borsh.deserialize(
    GameSchema,
    GameState,
    accountInfo.data,
  );

  for (let row = 0; row < 3; row++) {
    for (let col = 0; col < 3; col++) {
      let fieldContent = gameState.play_field[row * 3 + col];
      let drawSign = '*'
      if (fieldContent == 1) {
        drawSign = 'X'
      }
      if (fieldContent == 2) {
        drawSign = '0'
      }
      process.stdout.write(drawSign)
    }
    process.stdout.write('\n')
  }
}
