/// This file contains a wasm_bindgen interface to the chess engine core
use wasm_bindgen::prelude::*;
use engine_core::core;

#[wasm_bindgen]
pub struct ChessEngine {
    counter: usize
}

/// Wrap the chess engine with wasm_bindgen
#[wasm_bindgen]
impl ChessEngine {

    /// Create a new chess engine wrapper
    pub fn new() -> ChessEngine {
        ChessEngine { counter: 0}
    }

    /// Load ROM data to the emulator
    pub fn test(&mut self) -> String  {
        return "Hello World!".to_string();
    }

    pub fn get_counter(&self) -> usize {
        return self.counter;
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }
}
