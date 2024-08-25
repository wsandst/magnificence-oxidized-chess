
use std::iter::zip;

use super::{constants::*, Board, Color, Piece};

impl Board {
    //Evaluates total piece value for ```COLOR```.
    fn piece_values_for<const COLOR: bool>(&self) -> i32 {
        let pieces = match COLOR {
            WHITE => Piece::white_pieces(),
            BLACK => Piece::black_pieces()
        };
        zip(PIECE_VALUES, pieces)
            .map(|(value, piece)|value * (self.piece_sets[piece.to_u8() as usize].count_ones() as i32)).sum()
    }

    pub fn eval(&self) -> i32 {
        let result = self.piece_values_for::<WHITE>() - self.piece_values_for::<BLACK>();
        match self.current_player {
            Color::White => result,
            Color::Black => -result
        }
    }
}