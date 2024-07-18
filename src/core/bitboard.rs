use crate::core::*;
use std::fmt;
use lazy_static::lazy_static;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    piece_sets: [u64; 12],
    occupancy: u64,
    white_occupancy: u64,
    black_occupancy: u64,
    ep_history: Vec<u8>,
    castling_history: Vec<u8>,
    ep: u8,
    castling: u8
}

impl Board {
    pub fn new() -> Board {
        return Board {
            piece_sets: [0; 12],
            occupancy: 0,
            white_occupancy: 0,
            black_occupancy: 0,
            ep_history: Vec::new(),
            castling_history: Vec::new(),
            ep: 0,
            castling: 0
        }
        //return Board::new_from_fen(STARTING_POS_FEN);
    }

    pub fn new_from_fen(fen: &str) -> Board {
        todo!();
    }

    pub fn to_fen(&self) -> &str {
        return "Fen string here";
    }

    pub fn make_move(&mut self, mv: &Move) {
        //todo!()
    }

    pub fn unmake_move(&mut self, mv: &Move) {
        todo!()
    }

    pub fn set_piece(&mut self, x: u8, y: u8, piece: &Piece) {
        todo!()
    }

    pub fn get_piece(&self, x: u8, y: u8) -> &Piece {
        return &Piece::BlackKing;
    }

    // NOTE: Should probably use https://docs.rs/arrayvec/latest/arrayvec/ here in the future 
    pub fn get_moves(&self) -> Vec<Move> {
        let null_move = Move {from: 0, to: 0, promotion: 0, captured: 0};
        return vec![null_move, null_move, null_move, null_move, null_move];
    }
}

impl fmt::Display for Board {
    // String representation of board
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = String::with_capacity(64 + 8 + 64);
        board_string.push_str("\n  ABCDEFGH\n");
        for y in 0..8 {
            board_string.push_str(&format!("\n{} ", 8 - y));
            for x in 0..8 {
                let piece = self.get_piece(x, y);
                board_string.push(piece.as_char());
            }
        }
        board_string.push_str(&format!("\n\n{}", self.to_fen()));
        f.write_str(&board_string)
    }
}
// Lazy initialize some state at the beginning of the program
lazy_static! {
    pub static ref EXAMPLE_DATA: [u64; 12] = {
        let mut masks = [1,2,3,4,5,6,7,8,9,10,11,12];
        
        return masks;
    };
}
