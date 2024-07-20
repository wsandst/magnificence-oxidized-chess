use crate::core::*;
use lazy_static::lazy_static;
use rand::Rng;

mod formatting;

const CASTLING_RIGHTS_INDEX: usize = 13*64;
const EP_INDEX: usize = 13 * 64 + 4;
const PLAYER_INDEX: usize = 13 * 64 + 4 + 8;
const COLUMNS: [u64; 8] = {
    let mut masks = [0u64; 8];
    let mut i = 0;
    while i < 8 {
        let mut mask = 1u64 << i;
        let mut offset = 8;
        while offset < 64 {
            mask |= mask << offset;
            offset <<= 1;
        }
        masks[i] = mask;
        i += 1;
    }
    masks
};
const ROWS: [u64; 8] = {
    let mut masks = [0u64; 8];
    let mut i = 0;
    while i < 8 {
        let mut mask = 1u64 << (i * 8);
        let mut offset = 1;
        while offset < 8 {
            mask |= mask << offset;
            offset <<= 1;
        }
        masks[i] = mask;
        i += 1;
    }
    masks
};
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

    pub fn get_piece_pos(&self, x: usize, y: usize) -> Piece {
        return self.get_piece((y * 8 + x) as u8);
    }

    pub fn get_piece(&self, pos: u8) -> Piece {
        return self.mailboard[pos as usize];
    }

    pub fn get_current_player(&self) -> Color {
        return self.current_player;
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
        for (i, piece) in self.mailboard.iter().enumerate() {
            let piece_set = self.piece_sets[piece.to_u8() as usize];
            if piece_set & (1u64 << i) == 0 {
                panic!("Invalid board state. Piece {:?} at ({}, {}) was found in mailboard but not in the bitboard. \n Board: {}", 
                        piece, i % 8, i / 8, self);
            }
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
