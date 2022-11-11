use tic_tac_toe::{GameCell, GameStatus, GameInstruction, GameState};
use borsh_schema_derive::{construct_layouts, generate_output};


fn main() {

    let layouts = construct_layouts!(GameCell, GameStatus, GameInstruction, GameState);
    generate_output(&layouts, "./src/client").unwrap();
}
