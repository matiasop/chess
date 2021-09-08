use crate::logic::Board;
use crate::structs::PieceType;

pub fn print_board(board: &Board) {
    for (i, tile) in board.tiles.iter().enumerate() {
        let character = match tile.piece {
            None => "-",
            Some(piece) => match piece.color {
                true => match piece.piece_type {
                    PieceType::Pawn => "P",
                    PieceType::Knight => "N",
                    PieceType::Bishop => "B",
                    PieceType::Rook => "R",
                    PieceType::Queen => "Q",
                    PieceType::King => "K",
                },
                false => match piece.piece_type {
                    PieceType::Pawn => "p",
                    PieceType::Knight => "n",
                    PieceType::Bishop => "b",
                    PieceType::Rook => "r",
                    PieceType::Queen => "q",
                    PieceType::King => "k",
                },
            },
        };
        if i % 8 == 0 {
            println!();
        }
        print!("{}", character);
    }
}
