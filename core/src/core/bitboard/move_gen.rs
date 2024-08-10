#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};
// Use count_ones() for popcnt

mod pawns;
mod castling;
mod knights;
mod kings;
mod bishops;
mod rooks;

use bitboard::constants::*;
use num::PrimInt;

use crate::core::*;
use super::Board;

pub(crate) struct MovegenState {
    occupancy: u64,
    white_occupancy: u64,
    black_occupancy: u64,
    threatened_squares: u64,
    // Check related
    checks: u8,
    legal_targets: u64, // Used for legal targets during checks,
    rook_pins: u64,
    bishop_pins: u64
}

impl MovegenState {
    pub fn new(board: &Board) -> MovegenState {
        let occupancy = !(board.piece_sets[Piece::Empty.to_u8() as usize]);
        let white_occupancy = board.piece_sets[0] | board.piece_sets[1] | board.piece_sets[2] | board.piece_sets[3] | board.piece_sets[4] | board.piece_sets[5];
        let black_occupancy = white_occupancy ^ occupancy;
        
        let mut state = MovegenState {
            occupancy,
            white_occupancy,
            black_occupancy,
            threatened_squares: 0,
            checks: 0,
            legal_targets: 0,
            rook_pins: 0,
            bishop_pins: 0
        };
        match board.get_current_player() {
            Color::White => state.calculate_threatened_squares::<BLACK>(board),
            Color::Black => state.calculate_threatened_squares::<WHITE>(board),
        };
        return state;
    }

    fn calculate_threatened_squares<const COLOR: bool>(&mut self, board: &Board) {
        let offset: usize = match COLOR {
            WHITE => 0,
            BLACK => 6
        };
        let own_king = match COLOR {
            WHITE => board.piece_sets[Piece::BlackKing.to_u8() as usize],
            BLACK => board.piece_sets[Piece::WhiteKing.to_u8() as usize]
        };
        let own_occupancy = match COLOR {
            WHITE => self.black_occupancy,
            BLACK => self.white_occupancy
        };
        if own_king == 0 {
            self.legal_targets = !0;
            return;
        }
        let king_pos = own_king.trailing_zeros() as usize;
        let king_bishop_moves = board.runtime_constants.bishop_magic(king_pos, self.occupancy);
        let king_rook_moves = board.runtime_constants.rook_magic(king_pos, self.occupancy);

        let pawns = board.piece_sets[0 + offset];
        let queen = board.piece_sets[4 + offset];
        let mut bishops = board.piece_sets[1 + offset] | queen;
        let mut knights = board.piece_sets[2 + offset]; 
        let mut rooks = board.piece_sets[3 + offset] | queen;
        let king = board.piece_sets[5 + offset];

        self.calculate_pinned_masks(board, king_pos, own_occupancy, bishops, rooks, king_bishop_moves, king_rook_moves);

        self.threatened_squares = match COLOR {
            WHITE => ((pawns >> 7) & !COLUMNS[0]) | ((pawns >> 9) & !COLUMNS[7]),
            BLACK => ((pawns << 7) & !COLUMNS[7]) | ((pawns << 9) & !COLUMNS[0])
        };
        if self.threatened_squares & own_king > 0 {
            self.checks += 1;
            self.legal_targets |= match COLOR {
                WHITE => (((own_king << 7) & !COLUMNS[7]) | ((own_king << 9) & !COLUMNS[0])) & pawns,
                BLACK => (((own_king >> 7) & !COLUMNS[0]) | ((own_king >> 9) & !COLUMNS[7])) & pawns
            };
        }
        let occupancy = own_king ^ self.occupancy;
        while bishops > 0 {
            let pos: usize = bishops.trailing_zeros() as usize;
            let bishop_moves = board.runtime_constants.bishop_magic(pos, occupancy);
            if bishop_moves & own_king > 0 {
                self.legal_targets |= (1 << pos) | (board.runtime_constants.bishop_magic(pos, self.occupancy) & king_bishop_moves);
                self.checks += 1;
            }
            self.threatened_squares |= bishop_moves;
            bishops &= bishops - 1;
        }
        while rooks > 0 {
            let pos: usize = rooks.trailing_zeros() as usize;
            let rook_moves = board.runtime_constants.rook_magic(pos, occupancy);
            if rook_moves & own_king > 0 {
                self.legal_targets |= (1 << pos) | (board.runtime_constants.rook_magic(pos, self.occupancy) & king_rook_moves);
                self.checks += 1;
            }
            self.threatened_squares |= rook_moves;
            rooks &= rooks - 1;
        }
        while knights > 0 {
            let pos: usize = knights.trailing_zeros() as usize;
            let knight_moves = KNIGHT_MOVE_MASKS[pos];
            if knight_moves & own_king > 0 {
                self.checks += 1;
                self.legal_targets |= 1 << pos;
            }
            self.threatened_squares |= knight_moves;
            knights &= knights - 1;
        }
        if king > 0 {
            let pos: usize = king.trailing_zeros() as usize;
            self.threatened_squares |= KING_MOVE_MASKS[pos];
        }
        if self.checks == 0 {
            self.legal_targets = !0;
        }
    }

    /// Calculate legal pinned move masks
    fn calculate_pinned_masks(&mut self, board: &Board, king_pos: usize, own_occupancy: u64, bishops: u64, rooks: u64, king_bishop_moves: u64, king_rook_moves: u64) {
        let occupancy = self.occupancy & !((king_bishop_moves | king_rook_moves) & own_occupancy);

        let pinned_king_bishop_moves = board.runtime_constants.bishop_magic(king_pos, occupancy);
        let mut pinning_bishops = pinned_king_bishop_moves & bishops;
        while pinning_bishops > 0 {
            let pos: usize = pinning_bishops.trailing_zeros() as usize;
            self.bishop_pins |= board.runtime_constants.bishop_magic(pos, occupancy) & pinned_king_bishop_moves | (1 << pos);
            pinning_bishops &= pinning_bishops - 1;
        }

        let pinned_king_rook_moves = board.runtime_constants.rook_magic(king_pos, occupancy);
        let mut pinning_rooks = pinned_king_rook_moves & rooks;
        while pinning_rooks > 0 {
            let pos: usize = pinning_rooks.trailing_zeros() as usize;
            self.rook_pins |= board.runtime_constants.rook_magic(pos, occupancy) & pinned_king_rook_moves | (1 << pos);
            pinning_rooks &= pinning_rooks - 1;
        }
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
        if state.checks < 2 {
            self.generate_white_pawn_moves(moves, state);
            self.generate_white_knight_moves(moves, state);
            self.generate_white_bishop_like_moves(moves, state);
            self.generate_white_rook_like_moves(moves, state);
            self.generate_white_castling_moves(moves, state);
        }
        self.generate_white_king_moves(moves, state);
    }

    /// Generate valid moves for black
    pub(in crate::core) fn generate_moves_black(&self, moves : &mut Vec<Move>, state: &mut MovegenState) {
        if state.checks < 2 {
            self.generate_black_pawn_moves(moves, state);
            self.generate_black_knight_moves(moves, state);
            self.generate_black_bishop_like_moves(moves, state);
            self.generate_black_rook_like_moves(moves, state);
            self.generate_black_king_moves(moves, state);
        }
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