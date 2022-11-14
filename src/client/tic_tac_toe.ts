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
} from '@velas/web3';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';

import {
  getPayer,
  getRpcUrl,
  newAccountWithLamports,
  createKeypairFromFile,
} from './utils';

import {
  GameCell,
  GameCellTac,
  GameInstruction,
  GameInstructionGameReset,
  GameInstructionMakeTurn,
  GameState,
  GameStatus,
  GameStatusPlayerOneTurn,
  SCHEMA,
} from './schema';

/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'tic_tac_toe.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `velas program deploy dist/program/tic_tac_toe.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(
  PROGRAM_PATH,
  'tic_tac_toe-keypair.json',
);

/**
 * The expected size of game account.
 */
const GAME_STATE_SIZE = borsh.serialize(
  SCHEMA,
  new GameState({
    playField: new Array(9).fill(
      new GameCell({
        gameCellTac: new GameCellTac({}),
      }),
    ),
    status: new GameStatus({
      gameStatusPlayerOneTurn: new GameStatusPlayerOneTurn({}),
    }),
    playerOne: new PublicKey(new Uint8Array(32)),
    playerTwo: new PublicKey(new Uint8Array(32)),
  }),
).length;

/**
 * Establish a connection to the cluster
 */
export async function establishConnection(): Promise<Connection> {
  const rpcUrl = await getRpcUrl();
  const connection = new Connection(rpcUrl, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', rpcUrl, version);
  return connection;
}

/**
 * Establish an account to pay for everything
 * @param {Connection} connection - Established connection to the cluster
 * @param {string} [filePath] - Path to a file with Velas account keypair.
 * Takes default value from Velas CLI config file if argument is not specified.
 */
export async function establishPayer(
  connection: Connection,
  filePath: string | undefined = undefined,
): Promise<Keypair> {
  let fees = 0;
  const {feeCalculator} = await connection.getRecentBlockhash();

  // Calculate the cost to fund the game account
  fees += await connection.getMinimumBalanceForRentExemption(GAME_STATE_SIZE);

  // Calculate the cost of sending transactions
  fees += feeCalculator.lamportsPerSignature * 100; // wag

  let payer;

  try {
    payer = await getPayer(filePath);
  } catch (err) {
    // Fund a new payer via airdrop
    payer = await newAccountWithLamports(connection, fees);
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
    'VLX to pay for fees',
  );

  return payer;
}

/**
 * Check if the hello world BPF program has been deployed
 */
export async function checkProgram(
  connection: Connection,
  payer: Keypair,
): Promise<{programId: PublicKey; gamePubkey: PublicKey}> {
  // Read program id from keypair file
  let programId: PublicKey;
  try {
    const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
    programId = programKeypair.publicKey;
  } catch (err) {
    const errMsg = (err as Error).message;
    throw new Error(
      `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`velas program deploy dist/program/tic_tac_toe.so\``,
    );
  }

  // Check if the program has been deployed
  const programInfo = await connection.getAccountInfo(programId);
  if (programInfo === null) {
    if (fs.existsSync(PROGRAM_SO_PATH)) {
      throw new Error(
        'Program needs to be deployed with `velas program deploy dist/program/tic_tac_toe.so`',
      );
    } else {
      throw new Error('Program needs to be built and deployed');
    }
  } else if (!programInfo.executable) {
    throw new Error(`Program is not executable`);
  }
  console.log(`Using program ${programId.toBase58()}`);

  // Derive the address (public key) of a game account
  // from the program so that it's easy to find later.
  const GAME_SEED = 'hello';
  const gamePubkey = await PublicKey.createWithSeed(
    payer.publicKey,
    GAME_SEED,
    programId,
  );

  // Check if the game account has already been created
  const gameAccount = await connection.getAccountInfo(gamePubkey);
  if (gameAccount === null) {
    console.log('Creating game account: ', gamePubkey.toBase58());
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

  return {programId, gamePubkey};
}

export async function gameReset(
  connection: Connection,
  programId: PublicKey,
  gamePubkey: PublicKey,
  secondPlayer: PublicKey,
  payer: Keypair,
): Promise<any> {
  console.log('executing game reset:');

  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: gamePubkey, isSigner: false, isWritable: true},
      {pubkey: payer.publicKey, isSigner: true, isWritable: true},
    ],
    programId,
    data: Buffer.from(
      borsh.serialize(
        SCHEMA,
        new GameInstruction({
          gameInstructionGameReset: new GameInstructionGameReset({
            playerOne: payer.publicKey,
            playerTwo: secondPlayer,
          }),
        }),
      ),
    ),
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer],
  );
}

export async function makeTurn(
  connection: Connection,
  programId: PublicKey,
  gamePubkey: PublicKey,
  payer: Keypair,
  makeTurn: GameInstructionMakeTurn,
): Promise<void> {
  console.log(`executing makeTurn`);
  const data = borsh.serialize(
    SCHEMA,
    new GameInstruction({gameInstructionMakeTurn: makeTurn}),
  );
  const instruction = new TransactionInstruction({
    keys: [
      {pubkey: gamePubkey, isSigner: false, isWritable: true},
      {pubkey: payer.publicKey, isSigner: true, isWritable: true},
    ],
    programId,
    data: Buffer.from(data),
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
export async function reportPlayField(
  connection: Connection,
  gamePubkey: PublicKey,
): Promise<GameState> {
  const accountInfo = await connection.getAccountInfo(gamePubkey);
  if (accountInfo === null) {
    throw 'Error: cannot find the game account';
  }

  const gameState: GameState = borsh.deserialize(
    SCHEMA,
    GameState,
    accountInfo.data,
  );

  return gameState;
}

export function printGameState(gameState: GameState) {
  console.log('Game field:');
  for (let row = 0; row < 3; row++) {
    for (let col = 0; col < 3; col++) {
      let fieldContent = gameState.playField![row * 3 + col];
      let drawSign = '*';
      if (fieldContent.enum == 'gameCellTac') {
        drawSign = 'X';
      }
      if (fieldContent.enum == 'gameCellTic') {
        drawSign = '0';
      }
      process.stdout.write(drawSign + ' ');
    }
    process.stdout.write('\n');
  }

  console.log('Player one: ' + gameState.playerOne);
  console.log('Player two: ' + gameState.playerTwo);
  console.log('Status: ' + gameState.status!.enum);
}
