use crate::core::*;

mod formatting;
pub mod constants;
mod helpers;
mod move_gen;
use constants::*;
use move_gen::*;
use helpers::*;

#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};
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
    ep_history: Vec<u8>,
    castling_history: Vec<u8>,
    quiet_history: Vec<u8>,
    ep: u8,
    castling: u8,
    current_player: Color,
    quiet: u8,
    mailboard: [Piece; 64]
}

impl Board {
    pub fn empty() -> Board {
        let mut board = Board {
            piece_sets: [0; 13],
            hash_key: 0,
            ep_history: Vec::new(),
            castling_history: Vec::new(),
            quiet_history: Vec::new(),
            ep: 0,
            castling: 0,
            current_player: Color::White,
            quiet: 0,
            mailboard: [Piece::Empty; 64]
        };
        board.piece_sets[Piece::Empty.to_u8() as usize] = !(0u64);
        return board;
    }

    pub fn new() -> Board {
        return Board::from_fen(STARTING_POS_FEN);
    }

    pub fn make_move(&mut self, mv: &Move) {
        self.current_player = self.current_player.next_player();
        let piece_to_move = self.get_piece(mv.from);
        if piece_to_move == Piece::WhitePawn || piece_to_move == Piece::BlackPawn {
            if mv.promotion != Piece::Empty {
                // This piece is being promoted
                self.set_piece(mv.to, mv.promotion);
                self.set_piece(mv.from, Piece::Empty);
                return;
            }
            // Handle en passant
        }
        else if piece_to_move == Piece::WhiteKing || piece_to_move == Piece::BlackKing {
            // Black left side castling
            if mv.from == 4 && mv.to == 2 {
                self.set_piece(0, Piece::Empty);
                self.set_piece(2, Piece::BlackKing);
                self.set_piece(3, Piece::BlackRook);
                self.set_piece(4, Piece::Empty);
                return;
            }
            // Black right side castling
            else if mv.from == 4 && mv.to == 6 {
                self.set_piece(4, Piece::Empty);
                self.set_piece(5, Piece::BlackRook);
                self.set_piece(6, Piece::BlackKing);
                self.set_piece(7, Piece::Empty);
                return;
            }
            // White left side castling
            else if mv.from == 60 && mv.to == 62 {
                self.set_piece(60, Piece::Empty);
                self.set_piece(61, Piece::WhiteRook);
                self.set_piece(62, Piece::WhiteKing);
                self.set_piece(63, Piece::Empty);
                return;
            }
            // White right side castling
            else if mv.from == 60 && mv.to == 58 {
                self.set_piece(56, Piece::Empty);
                self.set_piece(58, Piece::WhiteKing);
                self.set_piece(59, Piece::WhiteRook);
                self.set_piece(60, Piece::Empty);
                return;
            }
        }
        self.set_piece(mv.to, piece_to_move);
        self.set_piece(mv.from, Piece::Empty);
    }

    pub fn unmake_move(&mut self, mv: &Move) {
        let moved_piece = self.get_piece(mv.to);
        self.current_player = self.current_player.next_player();

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
                self.set_piece(2, Piece::Empty);
                self.set_piece(3, Piece::Empty);
                self.set_piece(4, Piece::BlackKing);
                return;
            }
            // Black right side castling
            else if mv.from == 4 && mv.to == 6 {
                self.set_piece(4, Piece::BlackKing);
                self.set_piece(5, Piece::Empty);
                self.set_piece(6, Piece::Empty);
                self.set_piece(7, Piece::BlackRook);
                return;
            }
            // White left side castling
            else if mv.from == 60 && mv.to == 62 {
                self.set_piece(60, Piece::WhiteKing);
                self.set_piece(61, Piece::Empty);
                self.set_piece(62, Piece::Empty);
                self.set_piece(63, Piece::WhiteRook);
                return;
            }
            // White right side castling
            else if mv.from == 60 && mv.to == 58 {
                self.set_piece(56, Piece::WhiteRook);
                self.set_piece(58, Piece::Empty);
                self.set_piece(59, Piece::Empty);
                self.set_piece(60, Piece::WhiteKing);
                return;
            }
        }

        self.set_piece(mv.to, mv.captured);
        self.set_piece(mv.from, moved_piece);
    }

    pub fn set_piece_pos(&mut self, x: usize, y: usize, piece: &Piece) {
        self.set_piece((y * 8 + x) as u8, *piece)
    }

    pub fn set_piece(&mut self, pos: u8, piece: Piece) {
        let old_piece: Piece = self.mailboard[pos as usize];
        let piecenum = old_piece.to_u8() as usize;
        Board::unset_bit(&mut self.piece_sets[piecenum], pos);
        self.flip_zoobrist_piece(pos, old_piece);

        let piecenum = piece.to_u8() as usize;
        Board::set_bit(&mut self.piece_sets[piecenum], pos);
        self.flip_zoobrist_piece(pos, piece);

        self.mailboard[pos as usize] = piece;
    }

    pub fn get_piece_pos(&self, x: usize, y: usize) -> Piece {
        return self.get_piece((y * 8 + x) as u8);
    }

    pub fn get_piece(&self, pos: u8) -> Piece {
        return self.mailboard[pos as usize];
    }

    pub fn get_current_player(&self) -> Color {
        return self.current_player;
    }

    pub fn switch_current_player(&mut self) {
        self.current_player = self.current_player.next_player();
    }
}