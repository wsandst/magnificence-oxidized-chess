use crate::core::*;
use std::fmt;
use lazy_static::lazy_static;
use rand::Rng;

mod formatting;

const CASTLING_RIGHTS_INDEX: usize = 13*64;
const EP_INDEX: usize = 13 * 64 + 4;
const PLAYER_INDEX: usize = 13 * 64 + 4 + 8;

#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};
// Use count_ones() for popcnt

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
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
        let piece_to_move = *self.get_piece(mv.from as usize % 8, mv.from as usize / 8);
        self.set_piece_pos(mv.to as usize % 8, mv.to as usize / 8, &piece_to_move);
        self.set_piece_pos(mv.from as usize % 8, mv.from as usize / 8, &Piece::Empty);
        // Set en passant and such here too
    }

    pub fn unmake_move(&mut self, mv: &Move) {
        let moved_piece = *self.get_piece(mv.to as usize % 8, mv.to as usize / 8);
        self.set_piece_pos(mv.to as usize % 8, mv.to as usize / 8, &mv.captured);
        self.set_piece_pos(mv.from as usize % 8, mv.from as usize / 8, &moved_piece);
    }


    /// Generate a u8 representing castling rights from named booleans.
    pub fn generate_castling_u8(white_kingside: bool, white_queenside: bool, black_kingside: bool, 
            black_queenside: bool) -> u8 {
        return (white_kingside as u8) | ((white_queenside as u8) << 1) | 
            ((black_kingside as u8) << 2) | ((black_kingside as u8) << 3)
    }

    /// Set castling rights by named booleans.
    pub fn set_castling_bools(&mut self, white_kingside: bool, white_queenside: bool, black_kingside: bool, 
            black_queenside: bool) {
        self.set_castling(Board::generate_castling_u8(
            white_kingside, white_queenside, black_kingside, black_queenside
            )
        )
    }

    /// Gets the castling rights for given player. Returns queenside castling rights if 
    /// ```queenside``` is ```true```, kingside otherwise.
    pub fn get_castling(&self, player: Color, queenside: bool) -> bool {
        let color_offset = match player {
            Color::White => 0,
            Color::Black => 2
        };
        let index = color_offset + queenside as u8;
        return (self.castling & (1u8 << index)) > 0;
    }

    /// Set castling rights.
    pub fn set_castling(&mut self, new_val: u8) {
        let old_val = self.castling;
        self.castling = new_val;
        let mut difference = old_val ^ self.castling;
        while difference > 0 { 
            let index = difference.trailing_zeros() as usize;
            self.hash_key = self.hash_key ^ ZOOBRIST_KEYS[index + CASTLING_RIGHTS_INDEX];
            difference &= difference - 1;
        }
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

    pub fn get_piece(&self, x: usize, y: usize) -> &Piece {
        return &self.mailboard[y * 8 + x];
    }

    // NOTE: Should probably use https://docs.rs/arrayvec/latest/arrayvec/ here in the future 
    pub fn get_moves(&self) -> Vec<Move> {
        let null_move = Move {from: 0, to: 0, promotion: Piece::Empty, captured: Piece::Empty};
        return vec![null_move, null_move, null_move, null_move, null_move];
    }

    /// Sets the bit at ```pos``` to ```1```.
    pub fn set_bit(num: &mut u64, pos: u8){
        *num  = (*num) | (1u64 << pos);
    }
    
    /// Sets the bit at ```pos % 64``` to ```0```.
    pub fn unset_bit(num: &mut u64, pos: u8) {
        *num = (*num) & (!1u64).rotate_left(pos as u32);
    }

    /// Updates the zoobrist key based on the addition/removal of ```piece``` at ```pos```.
    fn flip_zoobrist_piece(&mut self, pos: u8, piece: Piece) {
        let index = (piece.to_u8() as usize) * 64 + (pos as usize);
        let key = ZOOBRIST_KEYS[index];
        self.hash_key = self.hash_key ^ key;
    }

    #[cfg(target_feature = "bmi2")]
    fn bmi_conditional_example() -> bool {
        return true;
    }

    #[cfg(not(target_feature = "bmi2"))]
    fn bmi_conditional_example() -> bool {
        return true;
    }

    /// Calculates the hash key from scratch. Used for debugging.
    pub fn calculate_hash(&self) -> u64 {
        let mut result = 0;
        for i in 0..64 {
            let index = i + (self.mailboard[i].to_u8() as usize) * 64;
            result = result ^ ZOOBRIST_KEYS[index];
        }
        return result;
    }

    /// Validate that the bitboard is in a valid state
    pub fn validate(&self) {
        // Validate that every mailboard piece has the bitboard correctly set
        let mut i = 0;
        for piece in self.mailboard {
            let piece_set = self.piece_sets[piece.to_u8() as usize];
            if piece_set & (1 << i) == 0 {
                panic!("Invalid board state. Piece {:?} at ({}, {}) was found in mailboard but not in the bitboard. \n Board: {}", 
                        piece, i % 8, i / 8, self);
            }
            i += 1;
        }
        // Valdiate that every bitboard piece has the mailboard correctly set
        for (piece, piece_set) in self.piece_sets.iter().enumerate() {
            for i in 0..64 {
                if piece_set & (1 << i) == 1 && self.mailboard[i] != Piece::from_u8(piece as u8) {
                    panic!("Invalid board state. Piece {:?} at ({}, {}) was found in bitboard but not in the mailboard. \n Board: {}", 
                            piece, i % 8, i / 8, self);
                }
            }
        }
        //Validate that the hask key is correct
        if self.calculate_hash() != self.hash_key {
            panic!("Invalid board state. Stored hash_key {}, calculated hashkey {} in boardstate \"{}\"", 
                self.hash_key, self.calculate_hash(), self.to_fen())
        }
    }
}

// Lazy initialize some state at the beginning of the program
lazy_static! {
    pub static ref ZOOBRIST_KEYS: [u64;13*64 + 4 + 8 + 1] = {
        let mut keys = [0u64; 13*64 + 4 + 8 + 1];
        let mut rng = rand::thread_rng();
        for i in 0..keys.len() {
            keys[i] = rng.gen::<u64>();
        }
        for i in 0..64 {
            keys[(Piece::Empty.to_u8() as usize) * 64 + i] = 0;
        }
        return keys;
    };
}
