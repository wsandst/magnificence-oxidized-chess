use move_list::MoveList;

use super::{Board, MovegenState};
use super::bitboard::constants::*;
use crate::core::*;

const PROMOTION_PIECES_WHITE : [Piece; 4] = [Piece::WhiteQueen, Piece::WhiteRook, Piece::WhiteBishop, Piece::WhiteKnight];
const PROMOTION_PIECES_BLACK : [Piece; 4] = [Piece::BlackQueen, Piece::BlackRook, Piece::BlackBishop, Piece::BlackKnight];

impl Board {
    fn extract_pawn_loop<const FROM_OFFSET: i8, const CAPTURES: bool, const WHITE: bool, const PROMOTION: bool>(&self, mut move_mask: u64, moves : &mut MoveList) {
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
                    moves.push(Move::new(
                        self,
                        (index as i8 + FROM_OFFSET) as u8,
                        index,
                        piece,
                        taken
                    ));
                }
            } else {
                moves.push(Move::new(
                    self,
                    (index as i8 + FROM_OFFSET) as u8,
                    index,
                    Piece::Empty,
                    taken
                ));
            }
        }
    }

    fn extract_pawn_moves<const FROM_OFFSET: i8, const CAPTURES: bool, const WHITE: bool>(&self, move_mask: u64, moves : &mut MoveList) {
        let promotion_mask = match WHITE {
            true => ROWS[0],
            false => ROWS[7]
        };
        self.extract_pawn_loop::<FROM_OFFSET, CAPTURES, WHITE, true>(move_mask & promotion_mask, moves);
        self.extract_pawn_loop::<FROM_OFFSET, CAPTURES, WHITE, false>(move_mask & (!promotion_mask), moves);
    }

    /// Extracts en passant moves. en passant moves only have at most 1 set bit in the move mask
    fn extract_ep_moves<const FROM_OFFSET: i8, const WHITE: bool>(&self, move_mask: u64, moves: &mut MoveList) {
        let ep_square_offset: i8 = match WHITE {
            true => 8,
            false => -8
        };
        if move_mask > 0 {
            let enemy_queen = match WHITE {
                true => self.piece_sets[Piece::BlackQueen.to_u8() as usize],
                false => self.piece_sets[Piece::WhiteQueen.to_u8() as usize]
            };
            let enemy_bishops = enemy_queen | match WHITE {
                true => self.piece_sets[Piece::BlackBishop.to_u8() as usize],
                false => self.piece_sets[Piece::WhiteBishop.to_u8() as usize] 
            };
            let enemy_rooks = enemy_queen | match WHITE {
                true => self.piece_sets[Piece::BlackRook.to_u8() as usize],
                false => self.piece_sets[Piece::WhiteRook.to_u8() as usize] 
            };
            let king_pos = (match WHITE {
                true => self.piece_sets[Piece::WhiteKing.to_u8() as usize],
                false => self.piece_sets[Piece::BlackKing.to_u8() as usize] 
            }).trailing_zeros() as usize;
            let to = move_mask.trailing_zeros() as u8;
            let from = (to as i8 + FROM_OFFSET) as u8;
            let ep_square = (to as i8 + ep_square_offset) as u8;
            let new_occupancy = (!self.piece_sets[Piece::Empty.to_u8() as usize]) ^ move_mask ^ (1u64 << from) ^ (1u64 << ep_square);
            let bishop_moves = self.runtime_constants.bishop_magic(king_pos , new_occupancy);
            let rook_moves = self.runtime_constants.rook_magic(king_pos, new_occupancy);
            if bishop_moves & enemy_bishops == 0 && rook_moves & enemy_rooks == 0 {
                moves.push(Move::new(self, 
                    from,
                    to,
                    Piece::Empty,
                    Piece::Empty
                ));
            }
        }
    }

    pub(in crate::core) fn generate_white_pawn_moves(&self, moves : &mut MoveList, state: &MovegenState) {
        let white_pawn_occupancy = self.get_piece_set(Piece::WhitePawn);
        let king_pos = self.get_piece_set(Piece::WhiteKing).trailing_zeros();
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
        let ep_mask = if self.ep == 0 {
            0
        } else {
            let ep_pawn = 1u64 << (self.ep - 1 + 24);
            let ep_square = 1u64 << ((self.ep - 1 + 16));
            if (ep_pawn | ep_square) & state.legal_targets > 0 {
                ep_square
            } else {
                0
            }
        };
        let legal_captures_with_ep = ((state.black_occupancy) & state.legal_targets) | ep_mask;
        
        // Captures left
        let left_captures_mask = ((white_pawn_occupancy & !state.rook_pins & !bishop_pins_left_diagonal) >> 9) & !(COLUMNS[7]) & legal_captures_with_ep;
        self.extract_pawn_moves::<9, true, true>(left_captures_mask & (!ep_mask), moves);
        self.extract_ep_moves::<9, true>(left_captures_mask & ep_mask, moves);

        // Captures right
        let right_captures_mask = ((white_pawn_occupancy & !state.rook_pins & !bishop_pins_right_diagonal) >> 7) & !(COLUMNS[0]) & legal_captures_with_ep;
        self.extract_pawn_moves::<7, true, true>(right_captures_mask & (!ep_mask), moves);
        self.extract_ep_moves::<7, true>(right_captures_mask & ep_mask, moves)
    }

    pub(in crate::core) fn generate_black_pawn_moves(&self, moves : &mut MoveList, state: &MovegenState) {
        let black_pawn_occupancy = self.get_piece_set(Piece::BlackPawn);

        let king_pos =self.get_piece_set(Piece::BlackKing).trailing_zeros();
        let rook_pins_horizontal = state.rook_pins & ROWS[(king_pos >> 3) as usize];
        let bishop_pins_left_diagonal = state.bishop_pins & RIGHT_LEFT_DIAGONALS[king_pos as usize];
        let bishop_pins_right_diagonal = state.bishop_pins & LEFT_RIGHT_DIAGONALS[king_pos as usize];

        // Move forward
        let forward_move_mask = ((black_pawn_occupancy & !state.bishop_pins & !rook_pins_horizontal) << 8) & !(state.occupancy);
        self.extract_pawn_moves::<-8, false, false>(forward_move_mask & state.legal_targets, moves);

        // Second rank double move
        let double_move_mask = ((forward_move_mask & ROWS[2]) << 8) & !(state.occupancy);
        self.extract_pawn_loop::<-16, false, false, false>(double_move_mask & state.legal_targets, moves);
        
        // Add EP square to occupancy as a virtual piece
        let ep_mask = if self.ep == 0 {
            0
        } else {
            let ep_pawn = 1u64 << (self.ep - 1 + 32);
            let ep_square = 1u64 << ((self.ep - 1 + 40));
            if (ep_pawn | ep_square) & state.legal_targets > 0 {
                ep_square
            } else {
                0
            }
        };
        let legal_captures_with_ep = (state.white_occupancy & state.legal_targets) | ep_mask;

        // Captures left
        let left_captures_mask = ((black_pawn_occupancy & !state.rook_pins & !bishop_pins_left_diagonal) << 9) & !(COLUMNS[0]) & legal_captures_with_ep;
        self.extract_pawn_moves::<-9, true, false>(left_captures_mask & (!ep_mask), moves);
        self.extract_ep_moves::<-9, false>(left_captures_mask & ep_mask, moves);

        // Captures right
        let right_captures_mask = ((black_pawn_occupancy & !state.rook_pins & !bishop_pins_right_diagonal) << 7) & !(COLUMNS[7]) & legal_captures_with_ep;
        self.extract_pawn_moves::<-7, true, false>(right_captures_mask & (!ep_mask), moves);
        self.extract_ep_moves::<-7, false>(right_captures_mask & ep_mask, moves);
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
        let mut moves = MoveList::empty();
        let movegen_state = MovegenState::new(&board, false);
        board.generate_white_pawn_moves(&mut moves, &movegen_state);

        assert_moves_eq_algebraic(&moves, &vec!["a2a3","b2b3","c2c3","d2d3","e2e3","f2f3","g2g3","h2h3",
                                               "a2a4","b2b4","c2c4","d2d4","e2e4","f2f4","g2g4","h2h4"]);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec!["a7a6","b7b6","c7c6","d7d6","e7e6","f7f6","g7g6","h7h6",
                                               "a7a5","b7b5","c7c5","d7d5","e7e5","f7f5","g7g5","h7h5"]);

        let board = Board::from_fen("r1bqkbnr/1P2pppp/5P2/2p3P1/1p5P/p7/PPPP2p1/RNBQKB1R", Rc::clone(&constant_state));
        let movegen_state = MovegenState::new(&board, false);
        moves.clear();
        board.generate_white_pawn_moves(&mut moves, &movegen_state);

        assert_moves_eq_algebraic(&moves, &vec![
            "b2b3","c2c3","d2d3","g5g6","h4h5",
            "c2c4", "d2d4",
            "b2a3", "f6e7", "f6g7",
            "b7a8q", "b7b8q", "b7c8q", "b7a8r", "b7b8r", "b7c8r", "b7a8b", "b7b8b", "b7c8b", "b7a8n", "b7b8n", "b7c8n"
        ]);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec![
            "h7h6","g7g6","e7e6","c5c4","b4b3",
            "h7h5", "e7e5",
            "a3b2", "e7f6", "g7f6",
            "g2f1q", "g2g1q", "g2h1q", "g2f1r", "g2g1r", "g2h1r", "g2f1b", "g2g1b", "g2h1b", "g2f1n", "g2g1n", "g2h1n"
        ]);   


        let board = Board::from_fen("K7/8/1p6/p1p5/8/P1P5/1P6/k7 b - - 0 1", Rc::clone(&constant_state));
        let movegen_state = MovegenState::new(&board, false);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec![
            "a5a4", "b6b5", "c5c4",
        ]);

        // En passant
        let board = Board::from_fen("K7/8/8/1pP5/5Pp1/8/8/k7 w - b6", Rc::clone(&constant_state));
        let movegen_state = MovegenState::new(&board, false);
        moves.clear();
        board.generate_white_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec![
            "c5c6", "c5b6", "f4f5",
        ]);

        let board = Board::from_fen("K7/8/8/1pP5/5Pp1/8/8/k7 b - f3", Rc::clone(&constant_state));
        let movegen_state = MovegenState::new(&board, false);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, &movegen_state);
        assert_moves_eq_algebraic(&moves, &vec![
            "g4g3", "g4f3", "b5b4",
        ]);
    }
}