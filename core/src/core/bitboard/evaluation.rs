
use std::iter::zip;

use super::{constants::*, Board, Color, Piece};

const PAWN_PIECE_SQUARE_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0    
];

const KNIGHT_PIECE_SQUARE_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const BISHOP_PIECE_SQUARE_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,    
];

const ROOK_PIECE_SQUARE_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

const QUEEN_PIECE_SQUARE_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const KING_PIECE_SQUARE_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

impl Board {
    //Evaluates total piece value for ```COLOR```.
    fn piece_values_for<const COLOR: bool>(&self) -> i32 {
        // Piece values
        let pieces = match COLOR {
            WHITE => Piece::white_pieces(),
            BLACK => Piece::black_pieces()
        };
        return zip(PIECE_VALUES, pieces)
            .map(|(value, piece)|value * (self.piece_sets[piece.to_u8() as usize].count_ones() as i32)).sum();
    }

    fn piece_square_values(&self) -> i32 {
        // Piece square values
        let mut i = 0;
        let mut piece_square_values = 0;
        for piece in self.mailboard {
            piece_square_values += match piece {
                Piece::Empty => 0,
                Piece::WhitePawn => PAWN_PIECE_SQUARE_TABLE[i],
                Piece::WhiteBishop => BISHOP_PIECE_SQUARE_TABLE[i],
                Piece::WhiteKnight => KNIGHT_PIECE_SQUARE_TABLE[i],
                Piece::WhiteRook => ROOK_PIECE_SQUARE_TABLE[i],
                Piece::WhiteQueen => QUEEN_PIECE_SQUARE_TABLE[i],
                Piece::WhiteKing => KING_PIECE_SQUARE_TABLE[i],
                Piece::BlackPawn => -PAWN_PIECE_SQUARE_TABLE[i],
                Piece::BlackBishop => -BISHOP_PIECE_SQUARE_TABLE[i],
                Piece::BlackKnight => -KNIGHT_PIECE_SQUARE_TABLE[i],
                Piece::BlackRook => -ROOK_PIECE_SQUARE_TABLE[i],
                Piece::BlackQueen => -QUEEN_PIECE_SQUARE_TABLE[i],
                Piece::BlackKing => -KING_PIECE_SQUARE_TABLE[i],
            };
            i += 1;
        }
        return piece_square_values;
    }

    pub fn eval(&self) -> i32 {
        let result = self.piece_values_for::<WHITE>() - self.piece_values_for::<BLACK>();
        match self.current_player {
            Color::White => result,
            Color::Black => -result
        }
    }
}