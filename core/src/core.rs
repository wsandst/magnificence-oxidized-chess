pub mod bitboard;
#[cfg(test)]
mod tests;

use strum_macros::EnumIter;
use num;
use num_derive::{FromPrimitive, ToPrimitive};

pub static STARTING_POS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, FromPrimitive, ToPrimitive)]
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
    pub captured : Piece
}


impl Piece {
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
}

fn pos_to_algebraic_pos(x: u8, y: u8) -> String {
    let col = 'a' as usize + x as usize;
    let row = '0' as usize + (7 - y as usize);
    return format!("{}{}", char::from(col as u8), char::from(row as u8));
}

impl Move {
    pub fn to_algebraic(&self) -> String {
        let from = pos_to_algebraic_pos(self.from % 8, self.from / 8);
        let to = pos_to_algebraic_pos(self.to % 8, self.to / 8);
        let mut algebraic_move = format!("{}{}", from, to);
        if self.promotion != Piece::Empty {
            algebraic_move.push(self.promotion.as_char());
        }
        return algebraic_move;
    }
}