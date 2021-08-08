/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  gameInit,
  makeTurn,
  reportPlayField,
} from './hello_world';

async function main() {
  console.log("Let's say hello to a Velas account...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Reset play field and fill internal state with pubkeys of players authorized to play
  await gameInit();

  // Make turn
  await makeTurn([0x00]); // game initialization

  await makeTurn([0x01, 0x01, 0x01]); // Make turn at x = 1, y = 1

  await reportPlayField();

  await makeTurn([0x01, 0x01, 0x02]); // Make turn at x = 1, y = 2

  await reportPlayField();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
