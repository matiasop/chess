#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub piece: Option<PieceType>,
    pub color: Option<bool>, // 0 is white, 1 is black
}

pub struct Tile {
    pub piece: Option<Piece>,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: bool,
}

impl Square {
    pub fn empty() -> Square {
        // returns empty square
        Square {
            piece: None,
            color: None,
        }
    }

    pub fn new(piece_type: PieceType, color: bool) -> Square {
        Square {
            piece: Some(piece_type),
            color: Some(color),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Move {
    pub start_square: isize,
    pub target_square: isize,
}