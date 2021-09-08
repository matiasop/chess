mod debug;
mod logic;
mod structs;
use debug::print_board;
use logic::Board;

fn main() {
    let board = Board::new();
    print_board(&board);
    // println!("{:#?}", board.move_data);
    board.generate_moves();
}
