use crate::structs::{Move, PieceType, Tile};
use core::panic;
use std::cmp::min;
use std::collections::HashMap;

pub struct Board {
    pub tiles: [Tile; 64],
    pub move_data: HashMap<isize, [isize; 8]>, // north, south, east, west, north_east, north_west, south_east, south_west
    pub color_to_move: bool,
}

impl Board {
    pub fn new() -> Board {
        let mut tiles = [Tile::empty(); 64];

        // Fill tiles with pieces and empty spaces
        for i in 0..64 {
            let color = i < 32;
            let tile = match i {
                0 | 7 | 56 | 63 => Tile::new(PieceType::Rook, color),
                1 | 6 | 57 | 62 => Tile::new(PieceType::Knight, color),
                2 | 5 | 58 | 61 => Tile::new(PieceType::Bishop, color),
                3 | 59 => Tile::new(PieceType::Queen, color),
                4 | 60 => Tile::new(PieceType::King, color),
                8..=15 | 48..=55 => Tile::new(PieceType::Pawn, color),
                _ => Tile::empty(),
            };
            tiles[i] = tile;
        }

        let move_data: HashMap<isize, [isize; 8]> = HashMap::new();
        let mut board = Board {
            tiles,
            move_data,
            color_to_move: true,
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

                let tile_index = col + 8 * row;

                self.move_data.insert(
                    tile_index,
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

        for t in 0..64 {
            let tile = self.tiles[t];
            match tile.piece {
                None => continue,
                Some(piece) => {
                    if piece.color == self.color_to_move {
                        let mut new_moves = match piece.piece_type {
                            PieceType::Rook | PieceType::Bishop | PieceType::Queen => {
                                self.generate_sliding_moves(t as isize, piece.piece_type)
                            }
                            _ => Vec::new(),
                        };
                        moves.append(&mut new_moves);
                    }
                }
            }
        }
        moves
    }

    pub fn generate_sliding_moves(&self, tile_idx: isize, piece_type: PieceType) -> Vec<Move> {
        let mut moves = Vec::new();
        let idxs = match piece_type {
            PieceType::Rook => 0..4,
            PieceType::Bishop => 4..8,
            PieceType::Queen => 0..8,
            _ => panic!(),
        };

        for direction_index in idxs {
            moves.append(&mut self.sliding_moves(tile_idx, direction_index));
        }
        // println!("{} {:?} {:?}", tile_idx, piece_type, moves);
        moves
    }

    fn sliding_moves(&self, tile_idx: isize, direction_index: usize) -> Vec<Move> {
        let mut moves = Vec::new();
        // north, south, east, west, north_east, north_west, south_east, south_west
        let directions_offset = [8, -8, 1, -1, 9, 7, -7, -9];

        for n in 0..self.move_data.get(&tile_idx).unwrap()[direction_index] {
            let target_tile = tile_idx + directions_offset[direction_index] * (n + 1);
            let piece_on_target_tile = self.tiles[target_tile as usize];

            match piece_on_target_tile.piece {
                None => {}
                Some(piece) => {
                    if piece.color == self.color_to_move {
                        // Blocked by friendly piece
                        break;
                    } else {
                        // Blocked by enemy piece
                        moves.push(Move {
                            start_tile: tile_idx,
                            target_tile,
                        });
                        break;
                    }
                }
            }
            moves.push(Move {
                start_tile: tile_idx,
                target_tile,
            });
        }

        moves
    }
}
