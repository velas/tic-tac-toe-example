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
  // console.log(Buffer.from([0x01, 0x00, 0x00]))
  console.log("Let's say hello to a Solana account...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Make turn
  await makeTurn([0x00]); // сброс поля

  await makeTurn([0x01, 0x01, 0x01]); // ход в клетку с коорд. 1х1

  await reportGameField();

  await makeTurn([0x01, 0x01, 0x02]); // ход в клетку с коорд. 1х2

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
