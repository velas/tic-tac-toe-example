/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  makeTurn as makeTurn,
  reportGame as reportGameField,
} from './hello_world';

async function main() {
  console.log("Let's say hello to a Velas account...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Make turn
  await makeTurn([0x00]); // game initialization

  await makeTurn([0x01, 0x01, 0x01]); // Make turn at x = 1, y = 1

  await reportGameField();

  await makeTurn([0x01, 0x01, 0x02]); // Make turn at x = 1, y = 2

  await reportGameField();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
