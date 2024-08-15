use super::{Board, MovegenState};
use crate::core::*;

impl Board {
    fn extract_rook_moves(&self, moves : &mut Vec<Move>, rook_like_occupancy: u64, same_color_occupancy: u64, state: &MovegenState) {
        let moveable_rooks = rook_like_occupancy & !state.bishop_pins;
        let mut unpinned_rooks = moveable_rooks & (!state.rook_pins);
        let legal_squares = !same_color_occupancy & state.legal_targets;
        while unpinned_rooks > 0 {
            let rook_index = unpinned_rooks.trailing_zeros() as usize;
            unpinned_rooks &= unpinned_rooks - 1;
            let target_mask = self.runtime_constants.rook_magic(rook_index, state.occupancy) & legal_squares;

            self.extract_moves_from_mask(moves, target_mask, rook_index as u8);
        }  

        let mut pinned_rooks = moveable_rooks & state.rook_pins;
        while pinned_rooks > 0 {
            let rook_index = pinned_rooks.trailing_zeros() as usize;
            pinned_rooks &= pinned_rooks - 1;
            let target_mask = self.runtime_constants.rook_magic(rook_index, state.occupancy) & legal_squares & state.rook_pins;
            self.extract_moves_from_mask(moves, target_mask, rook_index as u8);
        }  
    }

    /// Generate moves for white rooks + queen diagonals
    pub(in crate::core) fn generate_white_rook_like_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let rook_like_occupancy = self.get_piece_set(Piece::WhiteRook) | self.get_piece_set(Piece::WhiteQueen);
        self.extract_rook_moves(moves, rook_like_occupancy, state.white_occupancy, state);
    }

    /// Generate moves for black rooks + queen diagonals
    pub(in crate::core) fn generate_black_rook_like_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let rook_like_occupancy = self.get_piece_set(Piece::BlackRook) | self.get_piece_set(Piece::BlackQueen);
        self.extract_rook_moves(moves, rook_like_occupancy, state.black_occupancy, state);
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
        let mut moves = Vec::new();
        let runtime_constants = Rc::new(BOARD_CONSTANT_STATE.clone());
        // Check that knight moves are generated correctly in the starting position
        let board = Board::new(Rc::clone(&runtime_constants));
        let movegen_state = MovegenState::new(&board);
        board.generate_white_bishop_like_moves(&mut moves, &movegen_state);
        //assert_moves_eq_algebraic(&moves, &vec!["b1a3", "b1c3","g1f3", "g1h3"]);
    }
}