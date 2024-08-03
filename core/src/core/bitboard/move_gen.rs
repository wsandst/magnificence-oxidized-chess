#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};
// Use count_ones() for popcnt

mod pawns;
mod castling;
mod knights;

use crate::core::*;
use super::Board;

impl Board {

    /// Get all valid moves for this position. Pushes the moves to the mutable vector `moves` which is passed in.
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

    /// Generate valid moves for white
    fn generate_moves_white(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        self.generate_white_pawn_moves(moves, white_occupancy, black_occupancy);
        self.generate_white_knight_moves(moves, white_occupancy, black_occupancy);
        self.generate_white_castling_moves(moves, white_occupancy, black_occupancy);
    }

    /// Generate valid moves for black
    fn generate_moves_black(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        self.generate_black_pawn_moves(moves, white_occupancy, black_occupancy);
        self.generate_black_knight_moves(moves, white_occupancy, black_occupancy);
        self.generate_black_castling_moves(moves, white_occupancy, black_occupancy);
    }

    /// Get the color piece occupancy of the board. Returns a tuple of `(white_occupancy, black_occupancy)`
    pub fn get_occupancy(&self) -> (u64, u64) {
        let white_occupancy = self.piece_sets[0] | self.piece_sets[1] | self.piece_sets[2] | self.piece_sets[3] | self.piece_sets[4] | self.piece_sets[5];
        let black_occupancy = self.piece_sets[6] | self.piece_sets[7] | self.piece_sets[8] | self.piece_sets[9] | self.piece_sets[10] | self.piece_sets[11];
        return (white_occupancy, black_occupancy);
    }

    // Example of conditional functions based on BMI2
    #[cfg(target_feature = "bmi2")]
    fn bmi_conditional_example() -> bool {
        return true;
    }

    #[cfg(not(target_feature = "bmi2"))]
    fn bmi_conditional_example() -> bool {
        return true;
    }
}