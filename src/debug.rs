use crate::logic::Board;
use crate::structs::{PieceType, Square};

pub fn print_board(board: &Board) {
    for (i, square) in board.squares.iter().enumerate() {
        let character = match square {
            Square {
                piece: Some(PieceType::Pawn),
                color: Some(false),
            } => "P",
            Square {
                piece: Some(PieceType::Knight),
                color: Some(false),
            } => "N",
            Square {
                piece: Some(PieceType::Bishop),
                color: Some(false),
            } => "B",
            Square {
                piece: Some(PieceType::Rook),
                color: Some(false),
            } => "R",
            Square {
                piece: Some(PieceType::Queen),
                color: Some(false),
            } => "Q",
            Square {
                piece: Some(PieceType::King),
                color: Some(false),
            } => "K",
            Square {
                piece: Some(PieceType::Pawn),
                color: Some(true),
            } => "p",
            Square {
                piece: Some(PieceType::Knight),
                color: Some(true),
            } => "n",
            Square {
                piece: Some(PieceType::Bishop),
                color: Some(true),
            } => "b",
            Square {
                piece: Some(PieceType::Rook),
                color: Some(true),
            } => "r",
            Square {
                piece: Some(PieceType::Queen),
                color: Some(true),
            } => "q",
            Square {
                piece: Some(PieceType::King),
                color: Some(true),
            } => "k",
            _ => "-",
        };
        if i % 8 == 0 {
            println!();
        }
        print!("{}", character);
    }
}
