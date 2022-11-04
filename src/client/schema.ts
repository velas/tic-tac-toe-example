import {Struct, Enum, PublicKeyBE} from 'ts-borsh-schema';
export class GameCell extends Enum {
  gameCellEmpty: GameCellEmpty | undefined;
  gameCellTic: GameCellTic | undefined;
  gameCellTac: GameCellTac | undefined;
};

export class GameCellEmpty extends Struct {
};

export class GameCellTic extends Struct {
};

export class GameCellTac extends Struct {
};

export class GameStatus extends Enum {
  gameStatusUninitialized: GameStatusUninitialized | undefined;
  gameStatusPlayerOneTurn: GameStatusPlayerOneTurn | undefined;
  gameStatusPlayerTwoTurn: GameStatusPlayerTwoTurn | undefined;
  gameStatusGameEnd: GameStatusGameEnd | undefined;
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
  gameInstructionGameReset: GameInstructionGameReset | undefined;
  gameInstructionMakeTurn: GameInstructionMakeTurn | undefined;
};

export class GameInstructionGameReset extends Struct {
  playerOne: PublicKeyBE | undefined;
  playerTwo: PublicKeyBE | undefined;
};

export class GameInstructionMakeTurn extends Struct {
  row: number | undefined;
  col: number | undefined;
};

export class GameState extends Struct {
  playField: GameCell[] | undefined;
  status: GameStatus | undefined;
  playerOne: PublicKeyBE | undefined;
  playerTwo: PublicKeyBE | undefined;
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
			['playerOne', PublicKeyBE],
			['playerTwo', PublicKeyBE],
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
			['playerOne', PublicKeyBE],
			['playerTwo', PublicKeyBE],
                ],
            },
    ],

  [
    PublicKeyBE,
    {
      kind: 'struct',
      fields: [['value', [32]]],
    },
  ]

]);