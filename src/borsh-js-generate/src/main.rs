use tic_tac_toe::{GameCell, GameStatus, GameInstruction, GameState};
use agsol_borsh_schema::{construct_layouts, generate_output};


fn main() {

    let layouts = construct_layouts!(GameCell, GameStatus, GameInstruction, GameState);
    generate_output(&layouts, "./src/client").unwrap();
}
