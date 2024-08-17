use crate::core::*;

mod formatting;
pub mod constants;
mod helpers;
mod move_gen;
use constants::*;
use move_gen::MovegenState;
use move_list::MoveList;

#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};
use std::rc::Rc;
// Use count_ones() for popcnt

/// Represents a chess board.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    /// Piece masks in order determined by the enum ```Piece```. 
    /// 
    /// #### Encoding
    /// A8 (black queenside rook starting position) is bit 0. B8 (black queenside knight starting position)
    /// is bit 1. A7 is bit 8. H1 (white kingside rook starting position) is bit 63.
    piece_sets: [u64; 13],
    hash_key: u64,
    ep: u8,
    castling: u8,
    current_player: Color,
    quiet: u8,
    half_moves: u8,
    mailboard: [Piece; 64],
    runtime_constants: Rc<BitboardRuntimeConstants>
}

impl Board {
    pub fn empty(runtime_constants: Rc<BitboardRuntimeConstants>) -> Board {
        let mut board = Board {
            piece_sets: [0; 13],
            hash_key: 0,
            ep: 0,
            castling: 0,
            current_player: Color::White,
            quiet: 0,
            half_moves: 1,
            mailboard: [Piece::Empty; 64],
            runtime_constants
        };
        board.piece_sets[Piece::Empty.to_u8() as usize] = !(0u64);
        board.hash_key = board.calculate_hash();
        return board;
    }

    pub fn new(runtime_constants: Rc<BitboardRuntimeConstants>) -> Board {
        return Board::from_fen(STARTING_POS_FEN, runtime_constants);
    }

    pub fn make_move(&mut self, mv: &Move) {
        self.flip_player();
        let mut piece_to_move = self.get_piece(mv.from);
        let mut ep = 0;

        // Quiet moves
        if piece_to_move != Piece::WhitePawn && piece_to_move != Piece::BlackPawn 
                && mv.captured == Piece::Empty {
            self.quiet += 1;
        }
        else {
            self.quiet = 0;
        }

        if piece_to_move == Piece::WhitePawn || piece_to_move == Piece::BlackPawn {
            // Check if this move generates an ep square
            if piece_to_move == Piece::WhitePawn && mv.from.wrapping_sub(mv.to) == 16 {
                // Double move, need to set en passant square
                ep = (mv.from % 8) + 1;
            }
            else if piece_to_move == Piece::BlackPawn && mv.to.wrapping_sub(mv.from) == 16 {
                // Double move, need to set en passant square
                ep = (mv.from % 8) + 1;
            }
            // Check if this move performs en passant and zero the taken square
            else if piece_to_move == Piece::WhitePawn && self.ep > 0 && (self.ep + 16 - 1) == mv.to {
                self.set_piece((self.ep as usize + 24 - 1) as u8, Piece::Empty);
            }
            else if piece_to_move == Piece::BlackPawn && self.ep > 0 && (self.ep + 40 - 1) == mv.to {
                self.set_piece((self.ep as usize + 32 - 1) as u8, Piece::Empty);
            }
            else if mv.promotion != Piece::Empty {
                // This piece is being promoted
                piece_to_move = mv.promotion;
            }
        }
        else if self.ep > 0 {
            match mv.from {
                0 => self.set_one_castling_right::<BLACK, true, false>(),
                7 => self.set_one_castling_right::<BLACK, false, false>(),
                56 => self.set_one_castling_right::<WHITE, true, false>(),
                63 => self.set_one_castling_right::<WHITE, false, false>(),
                _ => (),
            }
            match mv.to {
                0 => self.set_one_castling_right::<BLACK, true, false>(),
                7 => self.set_one_castling_right::<BLACK, false, false>(),
                56 => self.set_one_castling_right::<WHITE, true, false>(),
                63 => self.set_one_castling_right::<WHITE, false, false>(),
                _ => (),
            }
            
            if piece_to_move == Piece::WhiteKing || piece_to_move == Piece::BlackKing {
                if piece_to_move == Piece::WhiteKing {
                    self.set_castling(self.castling & !(0b11))
                } else {
                    self.set_castling(self.castling & !(0b1100));
                }
                if mv.from == 4 && mv.to == 2 {
                    self.set_piece(0, Piece::Empty);
                    self.set_piece(3, Piece::BlackRook);
                }
                // Black king side castling
                else if mv.from == 4 && mv.to == 6 {
                    self.set_piece(5, Piece::BlackRook);
                    self.set_piece(7, Piece::Empty);
                }
                // White king side castling
                else if mv.from == 60 && mv.to == 62 {
                    self.set_piece(61, Piece::WhiteRook);
                    self.set_piece(63, Piece::Empty);
                }
                // White queen side castling
                else if mv.from == 60 && mv.to == 58 {
                    self.set_piece(56, Piece::Empty);
                    self.set_piece(59, Piece::WhiteRook);
                }
            }
        } 
        
        self.set_piece(mv.to, piece_to_move);
        self.set_piece(mv.from, Piece::Empty);
        self.set_ep(ep);
        self.half_moves += 1;
    }

    pub fn unmake_move(&mut self, mv: &Move) {
        self.half_moves -= 1;
        let moved_piece = self.get_piece(mv.to);
        self.flip_player();
        self.set_castling(mv.castling);
        self.set_ep(mv.ep);
        self.quiet = mv.quiet;

        if mv.promotion != Piece::Empty {
            // Undo promotion
            self.set_piece(mv.to, mv.captured);
            // Determine pawn color based on the current player color
            if self.current_player == Color::Black {
                self.set_piece(mv.from, Piece::BlackPawn);
            }
            else {
                self.set_piece(mv.from, Piece::WhitePawn);
            }
            return;
        }
        else if moved_piece == Piece::WhiteKing || moved_piece == Piece::BlackKing {
            // Black left side castling
            if mv.from == 4 && mv.to == 2 {
                self.set_piece(0, Piece::BlackRook);
                self.set_piece(3, Piece::Empty);
            }
            // Black right side castling
            else if mv.from == 4 && mv.to == 6 {
                self.set_piece(5, Piece::Empty);
                self.set_piece(7, Piece::BlackRook);
            }
            // White left side castling
            else if mv.from == 60 && mv.to == 62 {
                self.set_piece(61, Piece::Empty);
                self.set_piece(63, Piece::WhiteRook);
            }
            // White right side castling
            else if mv.from == 60 && mv.to == 58 {
                self.set_piece(56, Piece::WhiteRook);
                self.set_piece(59, Piece::Empty);
            }
        }
        else if moved_piece == Piece::WhitePawn && self.ep > 0 && ((mv.from - mv.to) % 8 != 0) && mv.captured == Piece::Empty {
            // Restore removed pawn from en passant
            self.set_piece((self.ep as usize + 24 - 1) as u8, Piece::BlackPawn);
        }
        else if moved_piece == Piece::BlackPawn && self.ep > 0 && ((mv.to - mv.from) % 8 != 0) && mv.captured == Piece::Empty {
            self.set_piece((self.ep as usize + 32 - 1) as u8, Piece::WhitePawn);
        }
        self.set_piece(mv.to, mv.captured);
        self.set_piece(mv.from, moved_piece);
    }

    pub fn set_piece_pos(&mut self, x: usize, y: usize, piece: &Piece) {
        self.set_piece((y * 8 + x) as u8, *piece)
    }

    pub fn set_piece(&mut self, pos: u8, piece: Piece) {
        let old_piece: Piece = self.get_piece(pos);
        let piecenum = old_piece.to_u8() as usize;
        Board::unset_bit(&mut self.piece_sets[piecenum], pos);
        self.flip_zoobrist_piece(pos, old_piece);

        let piecenum = piece.to_u8() as usize;
        Board::set_bit(&mut self.piece_sets[piecenum], pos);
        self.flip_zoobrist_piece(pos, piece);

        unsafe { *self.mailboard.get_unchecked_mut(pos as usize) = piece };
    }

    pub fn get_piece_pos(&self, x: usize, y: usize) -> Piece {
        return self.get_piece((y * 8 + x) as u8);
    }

    pub fn get_piece(&self, pos: u8) -> Piece {
        return unsafe { *self.mailboard.get_unchecked(pos as usize) };
    }

    pub fn get_current_player(&self) -> Color {
        return self.current_player;
    }

    pub fn get_hashkey(&self) -> u64 {
        return self.hash_key;
    }

    pub fn switch_current_player(&mut self) {
        self.flip_player();
    }

    pub fn get_game_status(&mut self) -> GameStatus {
        let mut legal_moves = MoveList::empty();
        let mut state = MovegenState::new(&self);
        match self.current_player {
            Color::White => self.generate_moves_white(&mut legal_moves, &mut state),
            Color::Black => self.generate_moves_black(&mut legal_moves, &mut state)
        }
        let no_legal_moves = legal_moves.len() == 0;
        if !no_legal_moves {
            return GameStatus::InProgress;
        }

        if no_legal_moves && !state.in_check() {
            return GameStatus::Stalemate
        }
        else if no_legal_moves && state.in_check() {
            return match self.get_current_player() {
                Color::Black => GameStatus::WhiteWon,
                Color::White => GameStatus::BlackWon
            }
        }

        return GameStatus::InProgress;
    }
}