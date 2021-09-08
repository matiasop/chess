#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub piece: Option<Piece>,
}

impl Tile {
    pub fn empty() -> Tile {
        // returns empty tile
        Tile { piece: None }
    }

    pub fn new(piece_type: PieceType, color: bool) -> Tile {
        Tile {
            piece: Some(Piece { piece_type, color }),
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
    pub start_tile: isize,
    pub target_tile: isize,
}
