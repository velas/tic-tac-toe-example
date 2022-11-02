/**
 * Tic-tac-toe
 */

import { PublicKey } from '@velas/web3'
import { program } from 'commander'
import { GameInstructionMakeTurn } from './schema'

import {
  establishConnection,
  establishPayer,
  checkProgram,
  gameReset,
  makeTurn,
  reportPlayField,
  printGameState
} from './tic_tac_toe'

import { getPayer } from './utils'

async function main() {
  program
    .command('show-key')
    .argument('[file_path]', "optional path to file with player's keypair used to sign transaction, or using default path from velas config file")
    .description('show the public key of player')
    .action(runShowKey)

  program
    .command('game-reset')
    .argument('<second_player>', "base58 encoded pubkey of the opponent you decided to play with")
    .argument('[file_path]', "optional path to file with player's keypair used to sign transaction, or using default path from velas config file")
    .description('reset play field and pubkeys of players')
    .action(runGameReset)

  program
    .command('game-state')
    .argument('<game_account>', 'account containing game state')
    .argument('[file_path]', "optional path to file with player's keypair used to sign transaction, or using default path from velas config file")
    .description('show state of game field')
    .action(runGameState)

  program
    .command('make-turn')
    .argument('<game_account>', 'account containing game state')
    .argument('<row>', '`x` coordinate, starting from top-left corner (0, 1 or 2)')
    .argument('<column>', '`y` coordinate, starting from top-left corner (0, 1 or 2)')
    .argument('[file_path]', "optional path to file with player's keypair used to sign transaction, or using default path from velas config file")
    .description('make turn at specified coordinates')
    .action(runMakeTurn)

  await program.parseAsync()
}

async function runShowKey(path: string | undefined) {
  let player = await getPayer(path)
  console.log(`Account's pubkey: ${player.publicKey.toBase58()}`)
}

async function runGameReset(secondPlayer: string, filePath: string | undefined) {
  let secondPlayerPubkey = new PublicKey(secondPlayer)

  // Establish connection to the cluster
  const connection = await establishConnection()

  // Determine who pays for the fees
  const payer = await establishPayer(connection, filePath)

  // Check if the program has been deployed
  let { programId, gamePubkey } = await checkProgram(connection, payer)

  // Reset play field and fill internal state with pubkeys of players authorized to play
  await gameReset(connection, programId, gamePubkey, secondPlayerPubkey, payer)

  console.log(`game's state pubkey: ${gamePubkey}`)
}

async function runGameState(gameAccount: string, filePath: string | undefined) {
  // Establish connection to the cluster
  const connection = await establishConnection()

  // Determine who pays for the fees
  const payer = await establishPayer(connection, filePath)

  // Check if the program has been deployed
  let { gamePubkey } = await checkProgram(connection, payer)

  let gameState = await reportPlayField(connection, gamePubkey)

  printGameState(gameState)
}

async function runMakeTurn(gameAccount: string, row: number, column: number, filePath: string) {
  // Establish connection to the cluster
  const connection = await establishConnection()

  // Determine who pays for the fees
  const payer = await establishPayer(connection, filePath)

  // Check if the program has been deployed
  let { programId } = await checkProgram(connection, payer)

  let gamePubkey = new PublicKey(gameAccount)

  console.log(`player "${payer.publicKey}" is making a move...`)

  // Make turn at specified coordinates
  await makeTurn(
	connection, programId, gamePubkey, payer, 
	new GameInstructionMakeTurn({row: row, col: column})
  )


  let gameState = await reportPlayField(connection, gamePubkey)

  printGameState(gameState)
}

main().then(
  () => process.exit(),
  err => {
    console.error(err)
    process.exit(-1)
  },
)
