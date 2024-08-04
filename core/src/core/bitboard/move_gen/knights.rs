use std::iter::Empty;

use super::Board;
use super::bitboard::constants::*;
use crate::core::*;



impl Board {
    fn extract_knight_moves(&self, moves : &mut Vec<Move>, knight_occupancy: u64, same_color_occupancy: u64) {
        let mut occupancy_mask = knight_occupancy;
        while occupancy_mask > 0 {
            let knight_index = occupancy_mask.trailing_zeros() as usize;
            occupancy_mask &= occupancy_mask - 1;
            let mut move_mask = KNIGHT_MOVE_MASKS[knight_index] & !(same_color_occupancy);
            while move_mask > 0 {
                let move_index = move_mask.trailing_zeros() as usize;
                move_mask &= move_mask - 1;
                moves.push(Move {
                    from: knight_index  as u8,
                    to: move_index as u8,
                    promotion: Piece::Empty,
                    captured: self.mailboard[move_index as usize]
                });
            }
        }
    }

    pub fn generate_white_knight_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let white_knight_occupancy = self.piece_sets[Piece::WhiteKnight.to_u8() as usize];
        self.extract_knight_moves(moves, white_knight_occupancy, white_occupancy);
    }

    pub fn generate_black_knight_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let black_knight_occupancy = self.piece_sets[Piece::BlackKnight.to_u8() as usize];
        self.extract_knight_moves(moves, black_knight_occupancy, black_occupancy);
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
        // Check that knight moves are generated correctly in the starting position
        let board = Board::new(Rc::clone(&runtime_constants));
        let (white_occupancy, black_occupancy) = board.get_occupancy();
        board.generate_white_knight_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["b1a3", "b1c3","g1f3", "g1h3"]);

        moves.clear();
        board.generate_black_knight_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["b8a6", "b8c6","g8f6", "g8h6"]);

        moves.clear();
        // Check that all knight moves are generated correctly in an open position
        let mut board = Board::empty(Rc::clone(&runtime_constants));
        board.generate_white_knight_moves(&mut moves, white_occupancy, black_occupancy);
        board.generate_black_knight_moves(&mut moves, white_occupancy, black_occupancy);
        assert_eq!(moves.len(), 0);

        board.set_piece_pos(3, 3, &Piece::WhiteKnight);
        board.generate_white_knight_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["d5c7", "d5b6","d5b4", "d5c3", "d5e3", "d5f4", "d5f6", "d5e7"]);

        moves.clear();
        board.generate_black_knight_moves(&mut moves, white_occupancy, black_occupancy);
        assert_eq!(moves.len(), 0);

        board.set_piece_pos(3, 3, &Piece::BlackKnight);
        board.generate_black_knight_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["d5b6","d5b4", "d5c3", "d5e3", "d5f4", "d5f6"]);
    }
}