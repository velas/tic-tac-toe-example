import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import Enum from "./extensions/enum";
import Struct from "./extensions/struct";
import { borshPublicKey } from "./extensions/publicKey";

borshPublicKey();

export class GameCell extends Enum {
    gameCellEmpty: GameCellEmpty;
    gameCellTic: GameCellTic;
    gameCellTac: GameCellTac;
};

export class GameCellEmpty extends Struct {
};

export class GameCellTic extends Struct {
};

export class GameCellTac extends Struct {
};

export class GameStatus extends Enum {
    gameStatusUninitialized: GameStatusUninitialized;
    gameStatusPlayerOneTurn: GameStatusPlayerOneTurn;
    gameStatusPlayerTwoTurn: GameStatusPlayerTwoTurn;
    gameStatusGameEnd: GameStatusGameEnd;
};

export class GameStatusUninitialized extends Struct {
};

export class GameStatusPlayerOneTurn extends Struct {
};

export class GameStatusPlayerTwoTurn extends Struct {
};

export class GameStatusGameEnd extends Struct {
};

export class GameInstruction extends Enum {
    gameInstructionGameReset: GameInstructionGameReset;
    gameInstructionMakeTurn: GameInstructionMakeTurn;
};

export class GameInstructionGameReset extends Struct {
    playerOne: PublicKey;
    playerTwo: PublicKey;
};

export class GameInstructionMakeTurn extends Struct {
    row: number;
    col: number;
};

export class GameState extends Struct {
    playField: GameCell[];
    status: GameStatus;
    playerOne: PublicKey;
    playerTwo: PublicKey;
};

export const SCHEMA = new Map<any, any>([
    [
            GameCell,
            {
                kind: 'enum', field: 'enum', values: [
			['gameCellEmpty', GameCellEmpty],
			['gameCellTic', GameCellTic],
			['gameCellTac', GameCellTac],
                ],
            },
    ],
    [
            GameCellEmpty,
            {
                kind: 'struct', fields: [
                ],
            },
    ],
    [
            GameCellTic,
            {
                kind: 'struct', fields: [
                ],
            },
    ],
    [
            GameCellTac,
            {
                kind: 'struct', fields: [
                ],
            },
    ],
    [
            GameStatus,
            {
                kind: 'enum', field: 'enum', values: [
			['gameStatusUninitialized', GameStatusUninitialized],
			['gameStatusPlayerOneTurn', GameStatusPlayerOneTurn],
			['gameStatusPlayerTwoTurn', GameStatusPlayerTwoTurn],
			['gameStatusGameEnd', GameStatusGameEnd],
                ],
            },
    ],
    [
            GameStatusUninitialized,
            {
                kind: 'struct', fields: [
                ],
            },
    ],
    [
            GameStatusPlayerOneTurn,
            {
                kind: 'struct', fields: [
                ],
            },
    ],
    [
            GameStatusPlayerTwoTurn,
            {
                kind: 'struct', fields: [
                ],
            },
    ],
    [
            GameStatusGameEnd,
            {
                kind: 'struct', fields: [
                ],
            },
    ],
    [
            GameInstruction,
            {
                kind: 'enum', field: 'enum', values: [
			['gameInstructionGameReset', GameInstructionGameReset],
			['gameInstructionMakeTurn', GameInstructionMakeTurn],
                ],
            },
    ],
    [
            GameInstructionGameReset,
            {
                kind: 'struct', fields: [
			['playerOne', 'publicKey'],
			['playerTwo', 'publicKey'],
                ],
            },
    ],
    [
            GameInstructionMakeTurn,
            {
                kind: 'struct', fields: [
			['row', 'u8'],
			['col', 'u8'],
                ],
            },
    ],
    [
            GameState,
            {
                kind: 'struct', fields: [
			['playField', [GameCell, 9]],
			['status', GameStatus],
			['playerOne', 'publicKey'],
			['playerTwo', 'publicKey'],
                ],
            },
    ],
]);