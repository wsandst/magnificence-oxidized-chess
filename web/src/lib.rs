use engine_core::commands;
use engine_core::engine::ab_engine::StandardAlphaBetaEngine;
use engine_core::engine::{Engine, SearchMetadata, SearchMetadataCallback};
/// This file contains a wasm_bindgen interface to the chess engine core
use wasm_bindgen::prelude::*;
use engine_core::core::{Color, Move, Piece, STARTING_POS_FEN};
use engine_core::core::bitboard::*;
use serde::{Serialize, Deserialize};
use js_sys::Promise;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub struct ChessEngine {
    board: Board,
    white_player: Option<Box<dyn Engine>>,
    black_player: Option<Box<dyn Engine>>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct PiecePosition {
   pub x: usize,
   pub y: usize,
   pub piece: usize
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MoveWrapper {
   pub from_x: usize,
   pub from_y: usize,
   pub to_x: usize,
   pub to_y: usize,
   pub promotion: usize
}

impl MoveWrapper {
    pub fn from_move(mv: &Move) -> MoveWrapper {
        return MoveWrapper {
            to_x: (mv.to % 8) as usize, 
            to_y: (mv.to / 8) as usize, 
            from_x: (mv.from % 8) as usize, 
            from_y: (mv.from / 8) as usize, 
            promotion: (mv.promotion.to_u8()) as usize
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct SearchMetadataWrapper {
    depth: usize,
    eval: f64,
    pv: String
}

///
/// When to search and when not to?
/// Make a move => get current turn => if 

/// Wrap the chess engine with wasm_bindgen
#[wasm_bindgen]
impl ChessEngine {

    /// Create a new chess engine wrapper
    pub fn new() -> ChessEngine {
        ChessEngine { board: Board::from_fen(STARTING_POS_FEN), white_player: None, black_player: None }
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

    pub fn set_white_player(&mut self, engine_name: String) {
        if engine_name.to_lowercase() == "human" {
            self.white_player = None;
        }
        else {
            self.white_player = Some(Box::new(StandardAlphaBetaEngine::new()));
        }
    }

    pub fn set_black_player(&mut self, engine_name: String) {
        if engine_name.to_lowercase() == "human" {
            self.black_player = None;
        }
        else {
            self.black_player = Some(Box::new(StandardAlphaBetaEngine::new()));
        }
    }

    fn handle_search_metadata(metadata: SearchMetadata) {
        let pv = metadata.pv.iter().map(|&mv| mv.to_algebraic()).collect::<Vec<String>>().join(" ");
        let wrapped_metadata = SearchMetadataWrapper { depth: metadata.depth, eval: metadata.eval, pv};
        js_search_metadata_update(serde_wasm_bindgen::to_value(&wrapped_metadata).unwrap());
    }

    fn get_search_metadata_callback() -> SearchMetadataCallback {
        return Box::new(Self::handle_search_metadata);
    }

    pub async fn search(&mut self) -> JsValue {
        TimeoutFuture::new(1_000).await;
        if self.board.get_current_player() == Color::Black && self.black_player.is_some() {
            let black_player = self.black_player.as_mut().unwrap();
            let moves = ChessEngine::moves_to_return_moves(&black_player.search(&self.board, Self::get_search_metadata_callback()));
            return serde_wasm_bindgen::to_value(&moves).unwrap();
        }
        else if self.white_player.is_some() {
            let white_player = self.white_player.as_mut().unwrap();
            let moves = ChessEngine::moves_to_return_moves(&white_player.search(&self.board, Self::get_search_metadata_callback()));
            return serde_wasm_bindgen::to_value(&moves).unwrap();
        }
        return "".into();
    }

    fn moves_to_return_moves(moves: &Vec<Move> ) -> Vec<MoveWrapper> {
        return moves.iter().map(|&mv| MoveWrapper::from_move(&mv)).collect();
    }

    pub fn perft(&self, depth: usize) -> usize {
        let mut board_copy = self.board.clone();
        return commands::perft(depth, &mut board_copy);
    }
}

#[wasm_bindgen(raw_module = "../src/callbacks.js")]
extern "C" {
    fn js_search_metadata_update(metadata: JsValue);
}