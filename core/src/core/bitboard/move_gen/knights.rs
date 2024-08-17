use move_list::MoveList;

use super::{Board, MovegenState};
use super::bitboard::constants::*;
use crate::core::*;

impl Board {
    fn extract_knight_moves(&self, moves : &mut MoveList, knight_occupancy: u64, same_color_occupancy: u64, state: &MovegenState) {
        let mut occupancy_mask = knight_occupancy & !(state.bishop_pins | state.rook_pins);
        let legal_squares = !same_color_occupancy & state.legal_targets;
        while occupancy_mask > 0 {
            let knight_index = occupancy_mask.trailing_zeros() as usize;
            occupancy_mask &= occupancy_mask - 1;
            let move_mask = KNIGHT_MOVE_MASKS[knight_index] & legal_squares;
            self.extract_moves_from_mask(moves, move_mask, knight_index as u8);
        }
    }

    pub(in crate::core) fn generate_white_knight_moves(&self, moves : &mut MoveList, state: &MovegenState) {
        let white_knight_occupancy = self.get_piece_set(Piece::WhiteKnight);
        self.extract_knight_moves(moves, white_knight_occupancy, state.white_occupancy, state);
    }

    pub(in crate::core) fn generate_black_knight_moves(&self, moves : &mut MoveList, state: &MovegenState) {
        let black_knight_occupancy = self.get_piece_set(Piece::BlackKnight);
        self.extract_knight_moves(moves, black_knight_occupancy, state.black_occupancy, state);
    }
}

#[cfg(test)]
mod tests {
    use move_gen::MovegenState;

    use crate::core::tests::BOARD_CONSTANT_STATE;
    use crate::core::tests::assert_moves_eq_algebraic;

    use super::bitboard::*;

    #[test]
    fn test_knight_move_gen() {
        let mut moves = MoveList::empty();
        let runtime_constants = Rc::new(BOARD_CONSTANT_STATE.clone());
        // Check that knight moves are generated correctly in the starting position
        let board = Board::new(Rc::clone(&runtime_constants));
        let movegen_state = MovegenState::new(&board);
        board.generate_white_knight_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["b1a3", "b1c3","g1f3", "g1h3"]);

        moves.clear();
        board.generate_black_knight_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["b8a6", "b8c6","g8f6", "g8h6"]);

        moves.clear();
        // Check that all knight moves are generated correctly in an open position
        let mut board = Board::empty(Rc::clone(&runtime_constants));
        board.generate_white_knight_moves(&mut moves, &movegen_state);
        board.generate_black_knight_moves(&mut moves, &movegen_state);
        assert_eq!(moves.len(), 0);

        board.set_piece_pos(3, 3, &Piece::WhiteKnight);
        board.generate_white_knight_moves(&mut moves,  &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["d5c7", "d5b6","d5b4", "d5c3", "d5e3", "d5f4", "d5f6", "d5e7"]);

        moves.clear();
        board.generate_black_knight_moves(&mut moves, &movegen_state);
        assert_eq!(moves.len(), 0);

        board.set_piece_pos(3, 3, &Piece::BlackKnight);
        board.generate_black_knight_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["d5b6","d5b4", "d5c3", "d5e3", "d5f4", "d5f6"]);
    }
}