use std::iter::Empty;

use super::Board;
use super::bitboard::constants::*;
use crate::core::*;

impl Board {
    fn extract_king_moves(&self, moves : &mut Vec<Move>, king_index: usize, same_color_occupancy: u64) {
        let mut move_mask = KING_MOVE_MASKS[king_index] & !(same_color_occupancy);
        while move_mask > 0 {
            let move_index = move_mask.trailing_zeros() as usize;
            move_mask &= move_mask - 1;
            moves.push(Move {
                from: king_index  as u8,
                to: move_index as u8,
                promotion: Piece::Empty,
                captured: self.mailboard[move_index as usize]
            });
        }
    }

    pub fn generate_white_king_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let white_king_index = self.piece_sets[Piece::WhiteKing.to_u8() as usize].trailing_zeros() as usize;
        self.extract_king_moves(moves, white_king_index, white_occupancy);
    }

    pub fn generate_black_king_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let black_king_index = self.piece_sets[Piece::BlackKing.to_u8() as usize].trailing_zeros() as usize;
        self.extract_king_moves(moves, black_king_index, black_occupancy);
    }
}

#[cfg(test)]
mod tests {
    use crate::core::tests::BOARD_CONSTANT_STATE;
    use crate::core::tests::assert_moves_eq_algebraic;

    use super::bitboard::*;

    #[test]
    fn test_king_move_gen() {
        let mut moves = Vec::new();
        let runtime_constants = Rc::new(BOARD_CONSTANT_STATE.clone());
        let mut board = Board::empty(Rc::clone(&runtime_constants));
        board.set_piece_pos(3, 3, &Piece::WhiteKing);
        board.set_piece_pos(3, 4, &Piece::BlackKing);
        let (white_occupancy, black_occupancy) = board.get_occupancy();
        board.generate_white_king_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["d5c6", "d5d6", "d5e6", "d5c5", "d5e5", "d5c4", "d5d4", "d5e4"]);

        moves.clear();
        board.generate_black_king_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["d4c5", "d4d5", "d4e5", "d4c4", "d4e4", "d4c3", "d4d3", "d4e3"]);
    }
}