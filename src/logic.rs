use std::cmp::min;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub piece: Option<PieceType>,
    pub color: Option<bool>, // 0 is white, 1 is black
}

impl Square {
    fn empty() -> Square {
        // returns empty square
        Square {
            piece: None,
            color: None,
        }
    }

    fn new(piece_type: PieceType, color: bool) -> Square {
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

pub struct Move {
    start_square: isize,
    target_square: isize,
}

pub struct Board {
    pub squares: [Square; 64],
    pub move_data: HashMap<isize, [isize; 8]>, // north, south, east, west, north_east, north_west, south_east, south_west
    pub color_to_move: bool,
}

impl Board {
    pub fn new() -> Board {
        let mut squares = [Square::empty(); 64];

        // Fill squares with pieces and empty spaces
        squares[0] = Square::new(PieceType::Rook, true);
        for i in 0..64 {
            let color = i < 32;
            let square = match i {
                0 | 7 | 56 | 63 => Square::new(PieceType::Rook, color),
                1 | 6 | 57 | 62 => Square::new(PieceType::Knight, color),
                2 | 5 | 58 | 61 => Square::new(PieceType::Bishop, color),
                3 | 59 => Square::new(PieceType::Queen, color),
                4 | 60 => Square::new(PieceType::King, color),
                8..=15 | 48..=55 => Square::new(PieceType::Pawn, color),
                _ => Square::empty(),
            };
            squares[i] = square;
        }

        let move_data: HashMap<isize, [isize; 8]> = HashMap::new();
        let mut board = Board {
            squares,
            move_data,
            color_to_move: false,
        };
        board.precompute_move_data();

        board
    }

    fn precompute_move_data(&mut self) {
        for row in 0..8 {
            for col in 0..8 {
                // horizontal and vertical moves
                let north = 7 - row;
                let south = row;
                let east = 7 - col;
                let west = col;

                // diagonal moves
                let north_east = min(north, east);
                let north_west = min(north, west);
                let south_east = min(south, east);
                let south_west = min(south, west);

                let square_index = col + 8 * row;

                self.move_data.insert(
                    square_index,
                    [
                        north, south, east, west, north_east, north_west, south_east, south_west,
                    ],
                );
            }
        }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        // returns a vector with all the valid moves
        let mut moves = Vec::new();

        for square in 0..64 {
            let piece = self.squares[square];
            match piece.piece {
                None => continue,
                Some(piece_type) => {
                    if piece.color.unwrap() == self.color_to_move {
                        match piece_type {
                            PieceType::Rook | PieceType::Bishop | PieceType::Queen => {
                                self.generate_sliding_moves(square as isize, piece_type)
                            }
                            _ => println!("to do"),
                        }
                    }
                }
            }
        }
        moves
    }

    pub fn generate_sliding_moves(&self, square: isize, piece_type: PieceType) {
        let mut moves = Vec::new();
        // north, south, east, west, north_east, north_west, south_east, south_west
        let directions_offset = [8, -8, 1, -1, 9, 7, -7, -9];
        for direction_index in 0..8 {
            for n in 0..self.move_data.get(&square).unwrap()[direction_index] {
                let targetSquare = square + directions_offset[direction_index] * (n + 1);
                let piece_on_target_square = self.squares[targetSquare as usize];

                match piece_on_target_square.color {
                    None => {}
                    Some(c) => {
                        if c == self.color_to_move {
                            // Blocked by friendly piece
                            break;
                        } else {
                            // Blocked by enemy piece
                            moves.push(Move {
                                start_square: square,
                                target_square: targetSquare,
                            });
                            break;
                        }
                    }
                }
                moves.push(Move {
                    start_square: square,
                    target_square: targetSquare,
                });
            }
        }
    }
}

pub fn initialize_game() {
    println!("initialize game");
    let board = Board::new();
}
