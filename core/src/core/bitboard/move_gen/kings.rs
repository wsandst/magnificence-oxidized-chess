use super::{Board, MovegenState};
use super::bitboard::constants::*;
use crate::core::*;

impl Board {
    fn extract_king_moves(&self, moves : &mut Vec<Move>, king_index: usize, same_color_occupancy: u64) {
        if king_index == 64 { // No king
            return;
        }
        let move_mask = KING_MOVE_MASKS[king_index] & !(same_color_occupancy);
        self.extract_moves_from_mask(moves, move_mask, king_index as u8);
    }

    pub(in crate::core) fn generate_white_king_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let white_king_index = self.piece_sets[Piece::WhiteKing.to_u8() as usize].trailing_zeros() as usize;
        self.extract_king_moves(moves, white_king_index, state.white_occupancy);
    }

    pub(in crate::core) fn generate_black_king_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let black_king_index = self.piece_sets[Piece::BlackKing.to_u8() as usize].trailing_zeros() as usize;
        self.extract_king_moves(moves, black_king_index, state.black_occupancy);
    }
}

#[cfg(test)]
mod tests {
    use move_gen::MovegenState;

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
        let movegen_state = MovegenState::new(&board);
        board.generate_white_king_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["d5c6", "d5d6", "d5e6", "d5c5", "d5e5", "d5c4", "d5d4", "d5e4"]);

        moves.clear();
        board.generate_black_king_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["d4c5", "d4d5", "d4e5", "d4c4", "d4e4", "d4c3", "d4d3", "d4e3"]);
    }
}