mod debug;
mod logic;
mod structs;
use debug::print_board;
use logic::{initialize_game, Board};

fn main() {
    initialize_game();
    let board = Board::new();
    print_board(&board);
    println!("{:#?}", board.move_data);
    board.generate_moves();
}
