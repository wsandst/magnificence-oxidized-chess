use ab_engine::StandardAlphaBetaEngine;

use super::core::Move;
pub mod ab_engine;
use crate::core::bitboard::Board;

pub type SearchMetadataCallback = Box<dyn Fn(SearchMetadata) -> ()>;
pub type LogCallback = Box<dyn Fn(&str) -> ()>;

pub type ShouldAbortSearchCallback = Box<dyn Fn() -> bool>;


#[derive(Clone, PartialEq, Debug)]
pub struct SearchMetadata {
    pub depth: usize,
    pub eval: f64,
    pub pv: Vec<Move>
}

pub enum EngineType {
    Standard,
}

pub trait Engine {
    fn search(&mut self, board: &Board) -> Vec<Move>;
    fn get_name(&self) -> &'static str;
}

pub fn from_name(name: &str, update_metadata_callback: SearchMetadataCallback, info_callback: LogCallback, should_abort_callback: ShouldAbortSearchCallback) -> Box<dyn Engine> {
    return Box::new(
        match name {
            ab_engine::ENGINE_NAME => StandardAlphaBetaEngine::new(update_metadata_callback, info_callback, should_abort_callback),
            _ => panic!("Attempted to instantiate engine with invalid name: {}", name)
    });
}