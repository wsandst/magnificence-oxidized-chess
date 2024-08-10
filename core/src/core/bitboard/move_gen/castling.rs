use super::{Board, MovegenState};
use crate::core::*;
use crate::core::bitboard::constants::*;

/*const WHITE_QUEENSIDE_CASTLING_MASK: u64 = 0b11111 << 56;
const WHITE_KINGSIDE_CASTLING_MASK: u64 = 0b1111 << 60;
const BLACK_QUEENSIDE_CASTLING_MASK: u64 = 0b11111;
const BLACK_KINGSIDE_CASTLING_MASK: u64 = 0b1111 << 4;*/

const WHITE_QUEENSIDE_FREE_CASTLING_MASK: u64 = 0b01110 << 56;
const WHITE_KINGSIDE_FREE_CASTLING_MASK: u64 = 0b0110 << 60;
const BLACK_QUEENSIDE_FREE_CASTLING_MASK: u64 = 0b01110;
const BLACK_KINGSIDE_FREE_CASTLING_MASK: u64 = 0b0110 << 4;

const WHITE_KING_SQUARE: u8 = 60;
const BLACK_KING_SQUARE: u8 = 4;

impl Board {
    fn extract_castling_moves<const COLOR: bool>(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        // Get some constants based on color
        let (
            queenside_castling_offset, 
            kingside_castling_offset, 
            queenside_castling_king_pos, 
            kingside_castling_king_pos,
            queenside_mask,
            kingside_mask,
            king_square
        ) = match COLOR {
            WHITE => (1, 0, 58, 62, WHITE_QUEENSIDE_FREE_CASTLING_MASK, WHITE_KINGSIDE_FREE_CASTLING_MASK, WHITE_KING_SQUARE),
            BLACK => (3, 2, 2, 6, BLACK_QUEENSIDE_FREE_CASTLING_MASK, BLACK_KINGSIDE_FREE_CASTLING_MASK, BLACK_KING_SQUARE)
        };

        if state.checks > 0 {
            return;
        }
        let can_castle_queenside: u64 = ((self.castling >> queenside_castling_offset) & 1) as u64;
        let can_castle_kingside: u64 = ((self.castling >> kingside_castling_offset) & 1) as u64;
        let queenside_squares_legal = (((state.occupancy | state.threatened_squares) & queenside_mask) == 0) as u64;
        let kingside_squares_legal = (((state.occupancy | state.threatened_squares) & kingside_mask) == 0) as u64;
        let mut move_mask: u64 = ((queenside_squares_legal & can_castle_queenside) << queenside_castling_king_pos) | 
                                 ((kingside_squares_legal & can_castle_kingside) << kingside_castling_king_pos);

        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            move_mask &= move_mask - 1;
            moves.push(Move {
                from: king_square,
                to: index,
                promotion: Piece::Empty,
                captured: Piece::Empty
            })
        }
    }

    pub(in crate::core) fn generate_white_castling_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        return self.extract_castling_moves::<WHITE>(moves, state);
    }

    pub(in crate::core) fn generate_black_castling_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        return self.extract_castling_moves::<BLACK>(moves, state);
    }
}

#[cfg(test)]
mod tests {
    use move_gen::MovegenState;

    use crate::core::tests::BOARD_CONSTANT_STATE;
    use crate::core::tests::assert_moves_eq_algebraic;

    use super::bitboard::*;

    #[test]
    fn test_castling_move_gen() {
        let mut moves = Vec::new();
        let runtime_constants = Rc::new(BOARD_CONSTANT_STATE.clone());
        // Check that the castling moves are not generated if blocked in the starting position
        let board = Board::new(Rc::clone(&runtime_constants));
        let movegen_state = MovegenState::new(&board);
        board.generate_white_castling_moves(&mut moves, &movegen_state);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 0);
        board.generate_black_castling_moves(&mut moves, &movegen_state);
        assert_eq!(moves.len(), 0);

        // Check that the castling moves generate when not blocked
        let mut board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1 ", Rc::clone(&runtime_constants));
        let movegen_state = MovegenState::new(&board);
        board.generate_white_castling_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["e1c1", "e1g1"]);
        moves.clear();
        board.switch_current_player();
        let movegen_state = MovegenState::new(&board);
        board.generate_black_castling_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["e8c8", "e8g8"]);
        moves.clear();
        board.validate();

        // Check that the castling flags are taken into account
        board.set_castling_bools(true, false, false, false);
        board.switch_current_player();
        let movegen_state = MovegenState::new(&board);
        board.generate_white_castling_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["e1g1"]);
        moves.clear();

        board.set_castling_bools(false, true, false, false);
        board.generate_white_castling_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["e1c1"]);
        moves.clear();

        board.set_castling_bools(false, false, true, false);
        board.switch_current_player();
        let movegen_state = MovegenState::new(&board);
        board.generate_black_castling_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["e8g8"]);
        moves.clear();

        board.set_castling_bools(false, false, false, true);
        board.generate_black_castling_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["e8c8"]);
        moves.clear();
        board.validate();
    }
}