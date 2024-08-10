use super::{Board, MovegenState};
use crate::core::*;

impl Board {
    fn extract_bishop_moves(&self, moves : &mut Vec<Move>, bishop_like_occupancy: u64, same_color_occupancy: u64, occupancy: u64) {
        let mut occupancy_mask = bishop_like_occupancy;
        while occupancy_mask > 0 {
            let bishop_index = occupancy_mask.trailing_zeros() as usize;
            occupancy_mask &= occupancy_mask - 1;
            let target_mask = self.runtime_constants.bishop_magic(bishop_index, occupancy) & !(same_color_occupancy);

            // Do king safety/pinned pieces here

            self.extract_moves_from_mask(moves, target_mask, bishop_index as u8);
        }  
    }

    /// Generate moves for white bishops + queen diagonals
    pub(in crate::core) fn generate_white_bishop_like_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let bishop_like_occupancy = self.piece_sets[Piece::WhiteBishop.to_u8() as usize] | self.piece_sets[Piece::WhiteQueen.to_u8() as usize];
        self.extract_bishop_moves(moves, bishop_like_occupancy, state.white_occupancy, state.occupancy);
    }

    /// Generate moves for black bishops + queen diagonals
    pub(in crate::core) fn generate_black_bishop_like_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let bishop_like_occupancy = self.piece_sets[Piece::BlackBishop.to_u8() as usize] |  self.piece_sets[Piece::BlackQueen.to_u8() as usize];
        self.extract_bishop_moves(moves, bishop_like_occupancy, state.black_occupancy, state.occupancy);
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