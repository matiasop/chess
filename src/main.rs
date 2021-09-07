mod debug;
mod logic;
use debug::print_board;
use logic::{initialize_game, Board, Move};

fn main() {
    initialize_game();
    let board = Board::new();
    print_board(&board);
    println!("{:#?}", board.move_data);
}
