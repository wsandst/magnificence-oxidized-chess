use std::iter::Empty;

use super::Board;
use super::bitboard::constants::*;
use crate::core::*;



impl Board {
    pub fn generate_white_knight_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let occupancy = white_occupancy | black_occupancy;
        for i in 0..64 {
            Self::print_bits(KNIGHT_MOVE_MASKS[i]);
        }
    }

    pub fn generate_black_knight_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let occupancy = white_occupancy | black_occupancy;

    }
}

#[cfg(test)]
mod tests {
    use crate::core::tests::BOARD_CONSTANT_STATE;
    use crate::core::tests::assert_moves_eq_algebraic;

    use super::bitboard::*;

    #[test]
    fn test_knight_move_gen() {
        let mut moves = Vec::new();
        let runtime_constants = Rc::new(BOARD_CONSTANT_STATE.clone());
        // Check that the castling moves are not generated if blocked in the starting position
        let board = Board::new(Rc::clone(&runtime_constants));
        let (white_occupancy, black_occupancy) = board.get_occupancy();
        board.generate_white_knight_moves(&mut moves, white_occupancy, black_occupancy);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 0);
    }
}