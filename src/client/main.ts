/**
 * Tic-tac-toe
 */

import { program } from 'commander'

import {
  establishConnection,
  establishPayer,
  checkProgram,
  gameReset,
  makeTurn,
  reportPlayField,
  printGameState
} from './hello_world'

async function main() {
  program
    .command('show-key')
    .argument('[path]', "optional path to file with player's keypair to sign transaction, or using default path from velas config")
    .description('show the public key of player')
    .action(runShowKey)

  program
    .command('game-state')
    .argument('[path]', "optional path to file with player's keypair to sign transaction, or using default path from velas config")
    .description('show state of game field')
    .action(runShowState)

  program
    .command('game-reset')
    .argument('<second_player>', "opponent's pubkey you decided to play with")
    .argument('[path]', "optional path to file with player's keypair to sign transaction, or using default path from velas config")
    .description('reset play field and pubkeys of players')
    .action(runResetGame)

  program
    .command('make-turn')
    .argument('<row>', '`x` coordinate, starting from top-left corner (0, 1 or 2)')
    .argument('<column>', '`y` coordinate, starting from top-left corner (0, 1 or 2)')
    .argument('[path]', "optional path to file with player's keypair to sign transaction, or using default path from velas config")
    .description('make turn at specified coordinates')
    .action(runMakeTurn)

  await program.parseAsync()

  console.log('Success')
}

async function runShowKey(path: string) {
  if (path) {
    console.log(path)
  } else {
    console.log('using default config path')
  }
}

async function runShowState() {
  console.log('Showing game state')

  // Establish connection to the cluster
  const connection = await establishConnection()

  // Determine who pays for the fees
  const payer = await establishPayer(connection)

  // Check if the program has been deployed
  let { programId, gamePubkey } = await checkProgram(connection, payer)

  let gameState = await reportPlayField(connection, gamePubkey)
  printGameState(gameState)
}

async function runResetGame(path: string) {
  if (path) {console.log('yes path')} else {console.log('no path')}

  // Establish connection to the cluster
  const connection = await establishConnection()

  // Determine who pays for the fees
  const payer = await establishPayer(connection)

  // Check if the program has been deployed
  let { programId, gamePubkey } = await checkProgram(connection, payer)

  // Reset play field and fill internal state with pubkeys of players authorized to play
  await gameReset(connection, programId, gamePubkey, payer)
}

async function runMakeTurn(row: number, column: number, path: string) {
  console.log('making turn')
  console.log('row: ' + row)
  console.log('column: ' + column)
  console.log('path: ' + path)

  // Establish connection to the cluster
  const connection = await establishConnection()

  // Determine who pays for the fees
  const payer = await establishPayer(connection)

  // Check if the program has been deployed
  let { programId, gamePubkey } = await checkProgram(connection, payer)

  // Make turn at specified coordinates
  await makeTurn(connection, programId, gamePubkey, payer, [0x01, row, column])

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
