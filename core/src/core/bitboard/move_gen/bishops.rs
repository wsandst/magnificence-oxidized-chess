use super::{Board, MovegenState};
use crate::core::*;

impl Board {
    fn extract_bishop_moves(&self, moves : &mut Vec<Move>, bishop_like_occupancy: u64, same_color_occupancy: u64, state: &MovegenState) {
        let moveable_bishops = bishop_like_occupancy & !state.rook_pins;
        let mut unpinned_bishops = moveable_bishops & (!state.bishop_pins);
        let legal_squares = !same_color_occupancy & state.legal_targets;
        while unpinned_bishops > 0 {
            let bishop_index = unpinned_bishops.trailing_zeros() as usize;
            unpinned_bishops &= unpinned_bishops - 1;
            let target_mask = self.runtime_constants.bishop_magic(bishop_index, state.occupancy) & legal_squares;

            self.extract_moves_from_mask(moves, target_mask, bishop_index as u8);
        }  

        let mut pinned_bishops = moveable_bishops & state.bishop_pins;
        while pinned_bishops > 0 {
            let bishop_index = pinned_bishops.trailing_zeros() as usize;
            pinned_bishops &= pinned_bishops - 1;
            let target_mask = self.runtime_constants.bishop_magic(bishop_index, state.occupancy) & legal_squares & state.bishop_pins;
            self.extract_moves_from_mask(moves, target_mask, bishop_index as u8);
        }  
    }

    /// Generate moves for white bishops + queen diagonals
    pub(in crate::core) fn generate_white_bishop_like_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let bishop_like_occupancy = self.piece_sets[Piece::WhiteBishop.to_u8() as usize] | self.piece_sets[Piece::WhiteQueen.to_u8() as usize];
        self.extract_bishop_moves(moves, bishop_like_occupancy, state.white_occupancy, state);
    }

    /// Generate moves for black bishops + queen diagonals
    pub(in crate::core) fn generate_black_bishop_like_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let bishop_like_occupancy = self.piece_sets[Piece::BlackBishop.to_u8() as usize] |  self.piece_sets[Piece::BlackQueen.to_u8() as usize];
        self.extract_bishop_moves(moves, bishop_like_occupancy, state.black_occupancy, state);
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