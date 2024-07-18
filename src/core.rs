pub mod bitboard;

#[cfg(test)]
mod tests;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from : u8,
    pub to : u8,
    pub promotion : u8,
    pub captured : u8
}