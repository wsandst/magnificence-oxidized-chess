#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};
// Use count_ones() for popcnt

mod pawns;
mod castling;
mod knights;
mod kings;

use bitboard::constants::*;

use crate::core::*;
use super::Board;

impl Board {

    pub fn threatened_squares<const COLOR: bool>(&self) -> u64 {
        let offset: usize = match COLOR {
            WHITE => 0,
            BLACK => 6
        };
        let pawns = self.piece_sets[0 + offset];
        let queen = self.piece_sets[4 + offset];
        let mut bishops = self.piece_sets[1 + offset] | queen;
        let mut knights = self.piece_sets[2 + offset]; 
        let mut rooks = self.piece_sets[3 + offset] | queen;
        let king = self.piece_sets[5 + offset];
        let mut result = match COLOR {
            WHITE => ((pawns >> 7) & !COLUMNS[0]) | ((pawns >> 9) & !COLUMNS[7]),
            BLACK => ((pawns << 7) & !COLUMNS[7]) | ((pawns << 9) & !COLUMNS[0])
        };
        let occupancy = !self.piece_sets[Piece::Empty.to_u8() as usize];
        while bishops > 0 {
            let pos: usize = bishops.trailing_zeros() as usize;
            result |= self.runtime_constants.bishop_magic(pos, occupancy);
            bishops &= bishops - 1;
        }
        while knights > 0 {
            let pos: usize = knights.trailing_zeros() as usize;
            result |= KNIGHT_MOVE_MASKS[pos];
            knights &= knights - 1;
        }
        while rooks > 0 {
            let pos: usize = rooks.trailing_zeros() as usize;
            result |= self.runtime_constants.rook_magic(pos, occupancy);
            rooks &= rooks - 1;
        }
        if king > 0 {
            let pos: usize = king.trailing_zeros() as usize;
            result |= KING_MOVE_MASKS[pos];
        }
        return result;
    }

    

    /// Get all valid moves for this position. Pushes the moves to the mutable vector `moves` which is passed in.
    pub fn get_moves(&self, moves: &mut Vec<Move>)  {
        let (white_occupancy, black_occupancy) = self.get_occupancy();
        match self.current_player {
            Color::White => self.generate_moves_white(moves, white_occupancy, black_occupancy),
            Color::Black => self.generate_moves_black(moves, white_occupancy, black_occupancy)
        }
    }

    /// Generate valid moves for white
    pub fn generate_moves_white(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        self.generate_white_pawn_moves(moves, white_occupancy, black_occupancy);
        self.generate_white_knight_moves(moves, white_occupancy, black_occupancy);
        self.generate_white_king_moves(moves, white_occupancy, black_occupancy);
        self.generate_white_castling_moves(moves, white_occupancy, black_occupancy);
    }

    /// Generate valid moves for black
    pub fn generate_moves_black(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        self.generate_black_pawn_moves(moves, white_occupancy, black_occupancy);
        self.generate_black_knight_moves(moves, white_occupancy, black_occupancy);
        self.generate_black_king_moves(moves, white_occupancy, black_occupancy);
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