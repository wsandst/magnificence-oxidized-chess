/// This file contains a wasm_bindgen interface to the chess engine core
use wasm_bindgen::prelude::*;
use engine_core::core::{Move, Piece};
use engine_core::core::bitboard::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct ChessEngine {
    board: Board
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct PiecePosition {
   pub x: usize,
   pub y: usize,
   pub piece: usize
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ReturnMove {
   pub from_x: usize,
   pub from_y: usize,
   pub to_x: usize,
   pub to_y: usize,
   pub promotion: usize
}

/// Wrap the chess engine with wasm_bindgen
#[wasm_bindgen]
impl ChessEngine {

    /// Create a new chess engine wrapper
    pub fn new() -> ChessEngine {
        ChessEngine { board: Board::new()}
    }

    pub fn get_piece(&self, x: usize, y: usize) -> usize  {
        return self.board.get_piece_pos(x, y).to_u8() as usize;
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece_value: usize) {
        let piece = Piece::from_u8(piece_value as u8);
        self.board.set_piece_pos(x, y, &piece);
    }

    pub fn get_pieces(&self) -> Vec<JsValue> {
        let mut pieces : Vec<JsValue> = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                let piece_value = self.board.get_piece_pos(x, y).to_u8();
                if piece_value != 12 {
                    let position = PiecePosition {x, y, piece: piece_value as usize};
                    pieces.push(serde_wasm_bindgen::to_value(&position).unwrap());
                }
            }
        }
        return pieces;
    }

    pub fn make_move(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize, promotion: usize) {
        if from_x == to_x && from_y == to_y {
            return;
        }
        let capture_piece = self.board.get_piece_pos(to_x, to_y);
        let mv = Move { 
            from: (from_y * 8 + from_x) as u8, 
            to: (to_y * 8 + to_x) as u8, 
            captured: capture_piece, 
            promotion: Piece::from_u8(promotion as u8)
        };
        self.board.make_move(&mv);
    }

    pub fn reset_board(&mut self) {
        self.board = Board::new();
    }

    pub fn get_allowed_engines() -> Vec<String> {
        // Iterate over some list of engines here
        return vec!["Magnificence".to_owned(), "Magnificence Experimental".to_owned()];
    }

    pub fn get_board_fen(&self) -> String {
        return self.board.to_fen();
    }

    pub fn search(&mut self) {
        
    }
}
