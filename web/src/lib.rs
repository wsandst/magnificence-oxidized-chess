/// This file contains a wasm_bindgen interface to the chess engine core
use wasm_bindgen::prelude::*;
use engine_core::core::{Move, Piece};
use engine_core::core::bitboard::*;

#[wasm_bindgen]
pub struct ChessEngine {
    counter: usize,
    board: Board
}

#[wasm_bindgen]
pub struct PiecePosition {
   pub x: usize,
   pub y: usize,
   pub piece: usize
}

/// Wrap the chess engine with wasm_bindgen
#[wasm_bindgen]
impl ChessEngine {

    /// Create a new chess engine wrapper
    pub fn new() -> ChessEngine {
        ChessEngine { counter: 0, board: Board::new()}
    }

    pub fn get_piece(&self, x: usize, y: usize) -> usize  {
        return self.board.get_piece(x, y).to_u8() as usize;
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece_value: usize) {
        let piece = Piece::from_u8(piece_value as u8);
        self.board.set_piece(x, y, &piece);
    }

    pub fn get_pieces(&self) -> Vec<PiecePosition> {
        let mut pieces : Vec<PiecePosition> = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                let piece_value = self.board.get_piece(x, y).to_u8();
                if piece_value != 0 {
                    pieces.push(PiecePosition {x, y, piece: piece_value as usize});
                }
            }
        }
        return pieces;
    }

    pub fn make_move(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) {
        let capture_piece = self.board.get_piece(to_x, to_y);
        let mv = Move { 
            from: (from_y * 8 + from_x) as u8, 
            to: (to_y * 8 + to_x) as u8, 
            captured: *capture_piece, 
            promotion: 0
        };
        self.board.make_move(&mv);
    }

    pub fn reset_board(&mut self) {
        self.board = Board::new();
    }
}
