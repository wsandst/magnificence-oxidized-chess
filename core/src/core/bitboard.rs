use crate::core::*;
use std::fmt;
use lazy_static::lazy_static;
use rand::Rng;

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

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::empty();
        let parts: Vec<&str> = fen.split(" ").collect();
        let pieces = parts[0];
        //let player_to_move = parts[1];
        //let castling = parts[2];
        //let en_passant = parts[3];
        //let half_move_counter = parts[4];
        //let full_move_counter = parts[5];

        let mut y: usize = 0;
        // Place pieces
        for row in pieces.split("/") {
            let mut x: usize = 0;
            for c in row.chars() {
                if c.is_digit(10) {
                    // Digit means empty spaces
                    let num = c.to_digit(10).unwrap() as usize;
                    x += num;
                }
                else {
                    // Map the character to the correct piece
                    let piece = Piece::from_char(c);
                    board.set_piece_pos(x, y, &piece);
                    x += 1;
                }
                if x >= 8 {
                    continue;
                }
            }
            y += 1;
        }

        return board;
    }

    pub fn to_fen(&self) -> String {
        let mut fen_string = String::with_capacity(64);
        let mut run_of_empty = 0;
        for (i, piece) in self.mailboard.iter().enumerate() {
            if run_of_empty > 0 && (*piece != Piece::Empty || i != 0 && i % 8 == 0) {
                fen_string.push_str(&format!("{}", run_of_empty));
                run_of_empty = 0;
            }
            if i != 0 && i % 8 == 0 {
                fen_string.push_str("/");
            }
            if *piece != Piece::Empty {
                fen_string.push(piece.as_char());
            }
            if *piece == Piece::Empty {
                run_of_empty += 1;
            }
        }
        if run_of_empty > 0 {
            fen_string.push_str(&format!("{}", run_of_empty));
        }
        return fen_string;
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

    fn flip_zoobrist_piece(&mut self, pos: u8, piece: Piece) {
        self.hash_key = self.hash_key ^ ZOOBRIST_KEYS[(piece.to_u8() as usize) * 64 + (pos as usize)];
    }

    #[cfg(target_feature = "bmi2")]
    fn bmi_conditional_example() -> bool {
        return true;
    }

    #[cfg(not(target_feature = "bmi2"))]
    fn bmi_conditional_example() -> bool {
        return true;
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
    }
}

impl fmt::Display for Board {
    // String representation of board
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = String::with_capacity(64 + 8 + 64);
        board_string.push_str("\n  ABCDEFGH\n");
        for y in 0..8 {
            board_string.push_str(&format!("\n{} ", 8 - y));
            for x in 0..8 {
                let piece = self.get_piece(x, y);
                board_string.push(piece.as_char());
            }
        }
        board_string.push_str(&format!("\n\n{}", self.to_fen()));
        f.write_str(&board_string)
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
