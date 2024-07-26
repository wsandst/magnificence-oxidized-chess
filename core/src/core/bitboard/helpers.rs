
use crate::core::*;
use bitboard::constants::*;
use bitboard;

impl Board {

    /// Sets the bit at `pos` to `1`.
    #[inline]
    pub fn set_bit(num: &mut u64, pos: u8){
        *num  = (*num) | (1u64 << pos);
    }
    
    /// Sets the bit at `pos % 64` to `0`.
    #[inline]
    pub fn unset_bit(num: &mut u64, pos: u8) {
        *num = (*num) & (!1u64).rotate_left(pos as u32);
    }

    /// Updates the zoobrist key based on the addition/removal of `piece` at `pos`.
    pub(super) fn flip_zoobrist_piece(&mut self, pos: u8, piece: Piece) {
        let index = (piece.to_u8() as usize) * 64 + (pos as usize);
        let key = ZOOBRIST_KEYS[index];
        self.hash_key = self.hash_key ^ key;
    }

    /// Calculates the hash key from scratch. Used for debugging.
    pub(super) fn calculate_hash(&self) -> u64 {
        let mut result = 0;
        for i in 0..64 {
            let index = i + (self.mailboard[i].to_u8() as usize) * 64;
            result = result ^ ZOOBRIST_KEYS[index];
        }
        return result;
    }

    /// Gets the castling rights for given player. Returns queenside castling rights if 
    /// `queenside` is `true`, kingside otherwise.
    pub(super) fn get_castling(&self, player: Color, queenside: bool) -> bool {
        let color_offset = match player {
            Color::White => 0,
            Color::Black => 2
        };
        let index = color_offset + queenside as u8;
        return (self.castling & (1u8 << index)) > 0;
    }

    /// Set castling rights.
    pub(super) fn set_castling(&mut self, new_val: u8) {
        let old_val = self.castling;
        self.castling = new_val;
        let mut difference = old_val ^ self.castling;
        while difference > 0 { 
            let index = difference.trailing_zeros() as usize;
            self.hash_key = self.hash_key ^ ZOOBRIST_KEYS[index + CASTLING_RIGHTS_INDEX];
            difference &= difference - 1;
        }
    }

    /// Generate a u8 representing castling rights from named booleans.
    pub(super) fn generate_castling_u8(white_kingside: bool, white_queenside: bool, black_kingside: bool, 
            black_queenside: bool) -> u8 {
        return (white_kingside as u8) | ((white_queenside as u8) << 1) | 
            ((black_kingside as u8) << 2) | ((black_kingside as u8) << 3)
    }

    /// Set castling rights by named booleans.
    pub(super) fn set_castling_bools(&mut self, white_kingside: bool, white_queenside: bool, black_kingside: bool, 
            black_queenside: bool) {
        self.set_castling(Board::generate_castling_u8(
            white_kingside, white_queenside, black_kingside, black_queenside
            )
        )
    }

    pub fn get_ep(&self) -> u8 {
        return self.ep;
    }

    pub fn get_quiet_moves(&self) -> u8 {
        return self.quiet;
    }

    pub fn get_half_moves(&self) -> u8 {
        return self.half_moves;
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
        // Validate that the hask key is correct
        if self.calculate_hash() != self.hash_key {
            panic!("Invalid board state. Stored hash_key {}, calculated hashkey {} in boardstate \"{}\"", 
                self.hash_key, self.calculate_hash(), self.to_fen())
        }
    }
}