use bitboard::BitboardRuntimeConstants;

use crate::core::*;
use std::{fmt, rc::Rc};
use super::Board;

impl Board {
    // Create a new board from a FEN string
    pub fn from_fen(fen: &str, runtime_constants: Rc<BitboardRuntimeConstants>) -> Board {
        let mut board = Board::empty(runtime_constants);
        let parts: Vec<&str> = fen.trim().split(" ").collect();

        let pieces = parts[0];
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
        // Extract current player color
        if parts.len() > 1 {
            board.current_player = Color::from_char(parts[1].chars().nth(0).unwrap());
        }
        // Castling
        if parts.len() > 2 {
            // let castling = parts[2];
            let white_kingside = parts[2].contains('K');
            let white_queenside = parts[2].contains('Q');
            let black_kingside = parts[2].contains('k');
            let black_queenside = parts[2].contains('q');
            board.set_castling_bools(white_kingside, white_queenside, black_kingside, black_queenside);
        }
        // EP
        if parts.len() > 3 && parts[3] != "-" {
            let (x, _) = algebraic_pos_to_pos(parts[3]);
            board.ep = x + 1;
        }
        // Quiet move number
        if parts.len() > 4 {
            board.quiet = str::parse(parts[4]).unwrap();
        }
        // Half moves
        if parts.len() > 5 {
            // Half moves
            board.half_moves = str::parse(parts[5]).unwrap();
        }

        return board;
    }

    /// Returns the current board as a FEN string
    pub fn to_fen(&self) -> String {
        let mut fen_string = String::with_capacity(64);
        // Pieces
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
        // Current player
        fen_string.push_str(&format!(" {}", self.current_player.to_char()));
        // Castling
        fen_string.push_str(&format!(" {}", &self.get_castling_str()));
        // EP
        fen_string.push_str(&format!(" {}", &self.get_ep_str()));
        // Quiet move number
        fen_string.push_str(&format!(" {}", self.quiet));
        // Full move number
        fen_string.push_str(&format!(" {}", self.half_moves));

        return fen_string;
    }

    /// Print the `bits` of a u64 integer, formatted as a chess board. Used for debugging.
    pub fn print_bits(bits: u64) {
        for i in 0..8 {
            println!("{:08b}", (bits.reverse_bits() >> ((7-i)*8)) as u8);
        }
        println!("");
    }

    pub fn get_castling_str(&self) -> String {
        let mut castling_str = "".to_string();
        if self.get_castling(Color::White, false) {
            castling_str.push('K');
        }
        if self.get_castling(Color::White, true) {
            castling_str.push('Q');
        }
        if self.get_castling(Color::Black, false) {
            castling_str.push('k');
        }
        if self.get_castling(Color::Black, true) {
            castling_str.push('q');
        }
        if castling_str.len() == 0 {
            return "-".to_string();
        }
        return castling_str;
    }

    pub fn get_ep_str(&self) -> String {
        let mut ep_str : String = "-".to_string();
        if self.get_ep() > 0 && self.current_player == Color::White {
            ep_str = pos_to_algebraic_pos(self.get_ep() - 1, 5);
        }
        else if self.get_ep() > 0 && self.current_player == Color::Black {
            ep_str = pos_to_algebraic_pos(self.get_ep() - 1, 2);
        }
        return ep_str;
    }
}

impl fmt::Display for Board {

    /// Return a string representation of the board. Used for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = String::with_capacity(64 + 8 + 64);
        board_string.push_str("\n  ABCDEFGH\n");
        for y in 0..8 {
            board_string.push_str(&format!("\n{} ", 8 - y));
            for x in 0..8 {
                let piece = self.get_piece_pos(x, y);
                board_string.push(piece.as_char());
            }
        }
        board_string.push_str(&format!("\n\n{}", self.to_fen()));
        f.write_str(&board_string)
    }
}