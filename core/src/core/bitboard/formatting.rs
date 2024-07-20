use crate::core::*;
use std::fmt;
use super::Board;

impl Board {
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