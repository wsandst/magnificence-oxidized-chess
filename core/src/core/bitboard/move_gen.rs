#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};
// Use count_ones() for popcnt

mod pawns;
mod castling;
mod knights;
mod kings;
mod bishops;

use crate::core::*;
use super::Board;

pub(crate) struct MovegenState {
    occupancy: u64,
    white_occupancy: u64,
    black_occupancy: u64,
}

impl MovegenState {
    pub fn new(board: &Board) -> MovegenState {
        let occupancy = !(board.piece_sets[Piece::Empty.to_u8() as usize]);
        let white_occupancy = board.piece_sets[0] | board.piece_sets[1] | board.piece_sets[2] | board.piece_sets[3] | board.piece_sets[4] | board.piece_sets[5];
        let black_occupancy = white_occupancy ^ occupancy;
        return MovegenState {
            occupancy,
            white_occupancy,
            black_occupancy
        };
    }
}


impl Board {

    /// Get all valid moves for this position. Pushes the moves to the mutable vector `moves` which is passed in.
    pub fn get_moves(&self, moves: &mut Vec<Move>)  {
        let mut state = MovegenState::new(&self);
        match self.current_player {
            Color::White => self.generate_moves_white(moves, &mut state),
            Color::Black => self.generate_moves_black(moves, &mut state)
        }
    }

    /// Generate valid moves for white
    pub(in crate::core) fn generate_moves_white(&self, moves : &mut Vec<Move>, state: &mut MovegenState) {
        self.generate_white_pawn_moves(moves, state);
        self.generate_white_knight_moves(moves, state);
        self.generate_white_bishop_moves(moves, state);
        self.generate_white_king_moves(moves, state);
        self.generate_white_castling_moves(moves, state);
    }

    /// Generate valid moves for black
    pub(in crate::core) fn generate_moves_black(&self, moves : &mut Vec<Move>, state: &mut MovegenState) {
        self.generate_black_pawn_moves(moves, state);
        self.generate_black_knight_moves(moves, state);
        self.generate_black_bishop_moves(moves, state);
        self.generate_black_king_moves(moves, state);
        self.generate_black_castling_moves(moves, state);
    }

    /// Helper function to extract moves from a move mask
    pub(in crate::core) fn extract_moves_from_mask(&self, moves: &mut Vec<Move>, mut mask: u64, from_index: u8) {
        while mask > 0 {
            let move_index = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            moves.push(Move {
                from: from_index  as u8,
                to: move_index as u8,
                promotion: Piece::Empty,
                captured: self.mailboard[move_index as usize]
            });
        };
    }
}