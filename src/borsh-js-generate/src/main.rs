use tic_tac_toe::{GameCell, GameStatus, GameInstruction, GameState};
use agsol_borsh_schema::{Layout, construct_layouts, generate_output};
use borsh::BorshSchema;



fn main() {

    let layouts = construct_layouts!(GameCell, GameStatus, GameInstruction, GameState);
    generate_output(&layouts, "./src/client").unwrap();
}
