pub mod bitboard;
pub mod move_list;

#[cfg(test)]
pub(crate) mod tests;

use std::{collections::binary_heap::Iter, fmt};

use bitboard::{constants::{BISHOP_VALUE, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE}, Board};
use strum_macros::EnumIter;
use num;
use num_derive::{FromPrimitive, ToPrimitive};

pub static STARTING_POS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, FromPrimitive, ToPrimitive)]
/// Represents a piece or square on the board
/// 
/// #### NOTE
/// Changing order of values changes their integer representation and may cause bugs 
/// in other parts of the code.
pub enum Piece {
    WhitePawn = 0,
    WhiteBishop,
    WhiteKnight,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackBishop,
    BlackKnight,
    BlackRook,
    BlackQueen,
    BlackKing,
    Empty,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, FromPrimitive, ToPrimitive)]
pub enum Color {
    White = 0,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from : u8,
    pub to : u8,
    pub promotion : Piece,
    pub captured : Piece,
    pub ep: u8,
    pub castling: u8,
    pub quiet: u8
}

const WHITE_PIECES: [Piece; 6] = [Piece::WhitePawn, Piece::WhiteBishop, Piece::WhiteKnight, Piece::WhiteRook, Piece::WhiteQueen, Piece::WhiteKing];
const BLACK_PIECES: [Piece; 6] = [Piece::BlackPawn, Piece::BlackBishop, Piece::BlackKnight, Piece::BlackRook, Piece::BlackQueen, Piece::BlackKing];

impl Piece {
    pub fn white_pieces() -> std::slice::Iter<'static, Piece> {
        WHITE_PIECES.iter()
    }

    pub fn black_pieces() -> std::slice::Iter<'static, Piece> {
        BLACK_PIECES.iter()
    }

    /// Generates an iterator over pieces following self of white
    /// Order is pawn -> bishop -> knight -> rook -> queen -> king
    pub fn following_pieces_white(self) -> std::slice::Iter<'static, Piece> { 
        WHITE_PIECES[self.to_u8() as usize..].iter()
    }

    /// Generates an iterator over pieces following self of black
    /// Order is pawn -> bishop -> knight -> rook -> queen -> king
    pub fn following_pieces_black(self) -> std::slice::Iter<'static, Piece> {
        BLACK_PIECES[((self.to_u8() as usize)-6)..].iter()
    }

    pub fn as_char(&self) -> char {
        match *self {
            Piece::WhitePawn => 'P',
            Piece::WhiteBishop => 'B',
            Piece::WhiteKnight => 'N',
            Piece::WhiteRook => 'R',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteKing => 'K',
            Piece::BlackPawn => 'p',
            Piece::BlackBishop => 'b',
            Piece::BlackKnight => 'n',
            Piece::BlackRook => 'r',
            Piece::BlackQueen => 'q',
            Piece::BlackKing => 'k',
            Piece::Empty => '.',
        }
    }
    const PIECES: [Piece; 6] = [Piece::WhitePawn, Piece::WhiteBishop, Piece::WhiteKnight, Piece::WhiteRook, Piece::WhiteQueen, Piece::WhiteKing];
    pub fn is_white(&self) -> bool {
        return self.to_u8() < 6;
    }

    pub fn is_black(&self) -> bool {
        let val: u8 = self.to_u8();
        return (val > 5) & (val < 12);
    }

    pub fn from_char(c: char) -> Piece {
        match c {
            'P' => Piece::WhitePawn,
            'B' => Piece::WhiteBishop,
            'N' => Piece::WhiteKnight,
            'R' => Piece::WhiteRook,
            'Q' => Piece::WhiteQueen,
            'K' => Piece::WhiteKing,
            'p' => Piece::BlackPawn,
            'b' => Piece::BlackBishop,
            'n' => Piece::BlackKnight, 
            'r' => Piece::BlackRook,
            'q' => Piece::BlackQueen,
            'k' => Piece::BlackKing,
            '.' => Piece::Empty,
            _ => panic!("Invalid character!"),
        }
    }

    pub fn from_u8(num: u8) -> Piece {
        return num::FromPrimitive::from_u8(num).unwrap();
    }

    pub fn to_u8(&self) -> u8 {
        return num::ToPrimitive::to_u8(self).unwrap();
    }

    pub fn eval_score(&self) -> i32 {
        match *self {
            Piece::WhitePawn => PAWN_VALUE,
            Piece::WhiteBishop => BISHOP_VALUE,
            Piece::WhiteKnight => KNIGHT_VALUE,
            Piece::WhiteRook => ROOK_VALUE,
            Piece::WhiteQueen => QUEEN_VALUE,
            Piece::WhiteKing => KING_VALUE,
            Piece::BlackPawn => PAWN_VALUE,
            Piece::BlackBishop => BISHOP_VALUE,
            Piece::BlackKnight => KNIGHT_VALUE,
            Piece::BlackRook => ROOK_VALUE,
            Piece::BlackQueen => QUEEN_VALUE,
            Piece::BlackKing => KING_VALUE,
            Piece::Empty => 0,
        }
    }
}

impl Color {
    pub fn next_player(&self) -> Color {
        return match *self {
            Color::Black => Color::White,
            Color::White => Color::Black
        };
    }

    pub fn from_char(c: char) -> Color {
        return match c {
            'w' => Color::White,
            'b' => Color::Black,
            _ => panic!("Invalid color character: {}", c)
        };
    }

    pub fn to_char(&self) -> char {
        return match *self {
            Color::White => 'w',
            Color::Black => 'b'
        };
    }

    pub const fn to_bool(&self) -> bool {
        return match *self {
            Color::White => true,
            Color::Black => false
        }
    }
}

pub fn pos_to_algebraic_pos(x: u8, y: u8) -> String {
    let col = 'a' as usize + x as usize;
    let row = '1' as usize + (7 - y as usize);
    return format!("{}{}", char::from(col as u8), char::from(row as u8));
}

pub fn algebraic_pos_to_pos(pos: &str) -> (u8, u8) {
    let x = pos.chars().nth(0).unwrap() as usize - 'a' as usize;
    let y = 7 - (pos.chars().nth(1).unwrap() as usize - '1' as usize);
    return (x as u8, y as u8);
}

impl Move {
    pub fn to_algebraic(&self) -> String {
        let from = pos_to_algebraic_pos(self.from % 8, self.from / 8);
        let to = pos_to_algebraic_pos(self.to % 8, self.to / 8);
        let mut algebraic_move = format!("{}{}", from, to);
        if self.promotion != Piece::Empty {
            let promotion_piece_offset = if self.promotion.is_white() {6} else {0};
            algebraic_move.push(Piece::from_u8(self.promotion.to_u8() + promotion_piece_offset).as_char());
        }
        return algebraic_move;
    }

    pub fn new(board: &Board, from: u8, to: u8, promotion: Piece, captured: Piece) -> Move {
        return Move {
            from,
            to,
            promotion,
            captured,
            ep: board.get_ep(),
            castling: board.get_castling_u8(),
            quiet: board.get_quiet_moves()
        }
    }

    pub fn from_pos(board: &Board, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Move {
        return Move::new(
            board, 
            (from_y * 8 + from_x) as u8,
            (to_y * 8 + to_x) as u8,
            board.get_piece_pos(to_x, to_y),
            Piece::Empty
        );
    }

    pub fn from_algebraic(board: &Board, algebraic: &str) -> Move {
        let from_x = algebraic.chars().nth(0).unwrap() as usize - 'a' as usize;
        let from_y = 7 - (algebraic.chars().nth(1).unwrap() as usize - '1' as usize);
        let to_x = algebraic.chars().nth(2).unwrap() as usize - 'a' as usize;
        let to_y = 7 - (algebraic.chars().nth(3).unwrap() as usize - '1' as usize);
        let promotion = if algebraic.len() > 4 {
            if board.get_piece_pos(from_x, from_y).is_white() {
                Piece::from_char(algebraic.to_uppercase().chars().nth(4).unwrap())
            }
            else {
                Piece::from_char(algebraic.to_lowercase().chars().nth(4).unwrap())
            }
        }   
        else {
            Piece::Empty
        };
        let mut mv = Move::from_pos(board, from_x, from_y, to_x, to_y);
        mv.promotion = promotion;
        mv.captured = board.get_piece_pos(to_x, to_y);
        return mv;
    }

    pub fn is_quiet(&self) -> bool {
        return self.captured == Piece::Empty;
    }
}

impl fmt::Display for Move {
    /// Return a string representation of the board. Used for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_algebraic())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter)]
pub enum GameStatus {
    InProgress,
    WhiteWon,
    BlackWon,
    Stalemate
}