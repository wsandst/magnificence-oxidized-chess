use super::{Board, MovegenState};
use super::bitboard::constants::*;
use crate::core::*;

const PROMOTION_PIECES_WHITE : [Piece; 4] = [Piece::WhiteQueen, Piece::WhiteRook, Piece::WhiteBishop, Piece::WhiteKnight];
const PROMOTION_PIECES_BLACK : [Piece; 4] = [Piece::BlackQueen, Piece::BlackRook, Piece::BlackBishop, Piece::BlackKnight];

impl Board {
    fn extract_pawn_loop<const FROM_OFFSET: i8, const CAPTURES: bool, const WHITE: bool, const PROMOTION: bool>(&self, mut move_mask: u64, moves : &mut Vec<Move>) {
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            move_mask &= move_mask - 1;
            let taken = match CAPTURES {
                false => Piece::Empty,
                true => self.mailboard[index as usize]
            };
            if PROMOTION {
                let promotion_pieces = match WHITE {
                    true => PROMOTION_PIECES_WHITE,
                    false => PROMOTION_PIECES_BLACK
                };
                for piece in promotion_pieces {
                    moves.push(Move {
                        to: index,
                        from: (index as i8 + FROM_OFFSET) as u8,
                        promotion: piece,
                        captured: taken
                    });
                }
            } else {
                moves.push(Move {
                    to: index,
                    from: (index as i8 + FROM_OFFSET) as u8,
                    promotion: Piece::Empty,
                    captured: taken
                });
            }
        }
    }

    fn extract_pawn_moves<const FROM_OFFSET: i8, const CAPTURES: bool, const WHITE: bool>(&self, move_mask: u64, moves : &mut Vec<Move>) {
        let promotion_mask = match WHITE {
            true => ROWS[0],
            false => ROWS[7]
        };
        self.extract_pawn_loop::<FROM_OFFSET, CAPTURES, WHITE, true>(move_mask & promotion_mask, moves);
        self.extract_pawn_loop::<FROM_OFFSET, CAPTURES, WHITE, false>(move_mask & (!promotion_mask), moves);
    }

    pub(in crate::core) fn generate_white_pawn_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let white_pawn_occupancy = self.piece_sets[Piece::WhitePawn.to_u8() as usize];

        let king_pos = self.piece_sets[Piece::WhiteKing.to_u8() as usize].trailing_zeros();
        let rook_pins_horizontal = state.rook_pins & ROWS[(king_pos >> 3) as usize];
        let bishop_pins_left_diagonal = state.bishop_pins & RIGHT_LEFT_DIAGONALS[king_pos as usize];
        let bishop_pins_right_diagonal = state.bishop_pins & LEFT_RIGHT_DIAGONALS[king_pos as usize];

        // Move forward
        let forward_move_mask = ((white_pawn_occupancy & !state.bishop_pins & !rook_pins_horizontal) >> 8) & !(state.occupancy);
        self.extract_pawn_moves::<8, false, true>(forward_move_mask & state.legal_targets, moves);

        // Second rank double move
        let double_move_mask = ((forward_move_mask & ROWS[5]) >> 8) & !(state.occupancy);
        self.extract_pawn_loop::<16, false, true, false>(double_move_mask & state.legal_targets, moves);

        // Add EP square to occupancy as a virtual piece
        let legal_captures_with_ep = match self.ep {
            0 => state.black_occupancy,
            _ => state.black_occupancy | (1 << (self.ep + 16 - 1)) 
        } & state.legal_targets;
        
        // Captures left
        let left_captures_mask = ((white_pawn_occupancy & !state.rook_pins & !bishop_pins_left_diagonal) >> 9) & !(COLUMNS[7]) & legal_captures_with_ep;
        self.extract_pawn_moves::<9, true, true>(left_captures_mask, moves);

        // Captures right
        let right_captures_mask = ((white_pawn_occupancy & !state.rook_pins & !bishop_pins_right_diagonal) >> 7) & !(COLUMNS[0]) & legal_captures_with_ep;
        self.extract_pawn_moves::<7, true, true>(right_captures_mask, moves);
    }

    pub(in crate::core) fn generate_black_pawn_moves(&self, moves : &mut Vec<Move>, state: &MovegenState) {
        let black_pawn_occupancy = self.piece_sets[Piece::BlackPawn.to_u8() as usize];

        let king_pos = self.piece_sets[Piece::BlackKing.to_u8() as usize].trailing_zeros();
        let rook_pins_horizontal = state.rook_pins & ROWS[(king_pos >> 3) as usize];
        let bishop_pins_left_diagonal = state.bishop_pins & RIGHT_LEFT_DIAGONALS[king_pos as usize];
        let bishop_pins_right_diagonal = state.bishop_pins & LEFT_RIGHT_DIAGONALS[king_pos as usize];

        // Move forward
        let forward_move_mask = ((black_pawn_occupancy & !state.bishop_pins & !rook_pins_horizontal) << 8) & !(state.occupancy);
        self.extract_pawn_moves::<-8, false, false>(forward_move_mask, moves);

        // Second rank double move
        let double_move_mask = ((forward_move_mask & ROWS[2]) << 8) & !(state.occupancy);
        self.extract_pawn_loop::<-16, false, false, false>(double_move_mask, moves);
        
        // Add EP square to occupancy as a virtual piece
        let white_occupancy_with_ep = match self.ep {
            0 => state.white_occupancy,
            _ => state.white_occupancy | (1 << (40 + self.ep - 1)) 
        };

        // Captures left
        let left_captures_mask = ((black_pawn_occupancy & !state.rook_pins & !bishop_pins_left_diagonal) << 9) & !(COLUMNS[0]) & white_occupancy_with_ep;
        self.extract_pawn_moves::<-9, true, false>(left_captures_mask, moves);

        // Captures right
        let right_captures_mask = ((black_pawn_occupancy & !state.rook_pins & !bishop_pins_right_diagonal) << 7) & !(COLUMNS[7]) & white_occupancy_with_ep;
        self.extract_pawn_moves::<-7, true, false>(right_captures_mask, moves);
    }
}

#[cfg(test)]
mod tests {
    use crate::core::tests::BOARD_CONSTANT_STATE;
    use move_gen::MovegenState;
    use tests::assert_moves_eq_algebraic;

    use super::bitboard::*;

    #[test]
    fn test_pawn_move_gen() {
        let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
        let board = Board::new(Rc::clone(&constant_state));
        let mut moves = Vec::new();
        let movegen_state = MovegenState::new(&board);
        board.generate_white_pawn_moves(&mut moves, &movegen_state);

        assert_moves_eq_algebraic(&moves, &vec!["a2a3","b2b3","c2c3","d2d3","e2e3","f2f3","g2g3","h2h3",
                                               "a2a4","b2b4","c2c4","d2d4","e2e4","f2f4","g2g4","h2h4"]);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["a7a6","b7b6","c7c6","d7d6","e7e6","f7f6","g7g6","h7h6",
                                               "a7a5","b7b5","c7c5","d7d5","e7e5","f7f5","g7g5","h7h5"]);

        let board = Board::from_fen("r1bqkbnr/1P2pppp/5P2/2p3P1/1p5P/p7/PPPP2p1/RNBQKB1R", Rc::clone(&constant_state));
        let movegen_state = MovegenState::new(&board);
        moves.clear();
        board.generate_white_pawn_moves(&mut moves, &movegen_state);

        assert_moves_eq_algebraic(&moves, &vec![
            "b2b3","c2c3","d2d3","g5g6","h4h5",
            "c2c4", "d2d4",
            "b2a3", "f6e7", "f6g7",
            "b7a8Q", "b7b8Q", "b7c8Q", "b7a8R", "b7b8R", "b7c8R", "b7a8B", "b7b8B", "b7c8B", "b7a8N", "b7b8N", "b7c8N"
        ]);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec![
            "h7h6","g7g6","e7e6","c5c4","b4b3",
            "h7h5", "e7e5",
            "a3b2", "e7f6", "g7f6",
            "g2f1q", "g2g1q", "g2h1q", "g2f1r", "g2g1r", "g2h1r", "g2f1b", "g2g1b", "g2h1b", "g2f1n", "g2g1n", "g2h1n"
        ]);   


        let board = Board::from_fen("8/8/1p6/p1p5/8/P1P5/1P6/8 b - - 0 1", Rc::clone(&constant_state));
        let movegen_state = MovegenState::new(&board);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec![
            "a5a4", "b6b5", "c5c4",
        ]);

        // En passant
        let board = Board::from_fen("8/8/8/1pP5/5Pp1/8/8/8 w - b6", Rc::clone(&constant_state));
        let movegen_state = MovegenState::new(&board);
        moves.clear();
        board.generate_white_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec![
            "c5c6", "c5b6", "f4f5",
        ]);

        let board = Board::from_fen("8/8/8/1pP5/5Pp1/8/8/8 b - f3", Rc::clone(&constant_state));
        let movegen_state = MovegenState::new(&board);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec![
            "g4g3", "g4f3", "b5b4",
        ]);
    }
}