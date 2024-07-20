pub mod bitboard;
#[cfg(test)]
mod tests;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use num;
use num_derive::{FromPrimitive, ToPrimitive};

pub static STARTING_POS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, FromPrimitive, ToPrimitive)]
pub enum Piece {
    Empty = 0,
    WhitePawn,
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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from : u8,
    pub to : u8,
    pub promotion : u8,
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
