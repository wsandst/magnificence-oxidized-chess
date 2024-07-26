use super::Board;
use super::bitboard::constants::*;
use crate::core::*;

impl Board {
    pub fn generate_white_castling_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let occupancy = white_occupancy & black_occupancy;
    }

    pub fn generate_black_castling_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let occupancy = white_occupancy & black_occupancy;
    }
}

#[cfg(test)]
mod tests {
    use tests::assert_moves_eq_algebraic;

    use super::bitboard::*;

    #[test]
    fn test_pawn_move_gen() {
        return;
        // Check that the castling moves are not generated if blocked in the starting position
        let runtime_constants = Rc::from(BitboardRuntimeConstants::create());
        let board = Board::new(Rc::clone(&runtime_constants));
        let (white_occupancy, black_occupancy) = board.get_occupancy();
        let mut moves = Vec::new();
        board.generate_white_castling_moves(&mut moves, white_occupancy, black_occupancy);
        assert_eq!(moves.len(), 0);
        board.generate_black_castling_moves(&mut moves, white_occupancy, black_occupancy);
        assert_eq!(moves.len(), 0);

        // Check that the castling moves generate when not blocked
        let mut board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1 ", Rc::clone(&runtime_constants));
        board.generate_white_castling_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["e1c1", "e1g1"]);
        moves.clear();
        board.generate_black_castling_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["e8c8", "e8g8"]);
        moves.clear();
        board.validate();

        // Check that the castling flags are taken into account
        board.set_castling_bools(true, false, false, false);
        board.generate_white_castling_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["e1g1"]);
        moves.clear();

        board.set_castling_bools(false, true, false, false);
        board.generate_white_castling_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["e1c1"]);
        moves.clear();

        board.set_castling_bools(false, false, true, false);
        board.generate_black_castling_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["e8g8"]);
        moves.clear();

        board.set_castling_bools(false, false, false, true);
        board.generate_black_castling_moves(&mut moves, white_occupancy, black_occupancy);
        assert_moves_eq_algebraic(&moves, &vec!["e8c8"]);
        moves.clear();
        board.validate();
    }
}