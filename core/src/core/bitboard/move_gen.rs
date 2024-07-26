use std::ptr::null;

mod pawns;

use crate::core::*;
use super::Board;

impl Board {
    // NOTE: Should probably use https://docs.rs/arrayvec/latest/arrayvec/ here in the future 
    pub fn get_moves(&self, moves: &mut Vec<Move>) -> (usize, usize)  {
        let (white_occupancy, black_occupancy) = self.get_occupancy();
        let current_end = moves.len();
        match self.current_player {
            Color::White => self.generate_moves_white(moves, white_occupancy, black_occupancy),
            Color::Black => self.generate_moves_black(moves, white_occupancy, black_occupancy)
        }
        let new_end = moves.len();
        return (current_end, new_end);
    }

    fn generate_moves_white(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        self.generate_white_pawn_moves(moves, white_occupancy, black_occupancy);
    }

    fn generate_moves_black(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        self.generate_black_pawn_moves(moves, white_occupancy, black_occupancy);
    }

    pub fn get_occupancy(&self) -> (u64, u64) {
        let white_occupancy = self.piece_sets[0] | self.piece_sets[1] | self.piece_sets[2] | self.piece_sets[3] | self.piece_sets[4] | self.piece_sets[5];
        let black_occupancy = self.piece_sets[6] | self.piece_sets[7] | self.piece_sets[8] | self.piece_sets[9] | self.piece_sets[10] | self.piece_sets[11];
        return (white_occupancy, black_occupancy);
    }
}