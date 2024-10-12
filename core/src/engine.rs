pub mod ab_engine;
pub mod random_engine;
pub mod move_sorting;
pub mod pv;

use std::time::Duration;

use ab_engine::StandardAlphaBetaEngine;
use random_engine::RandomEngine;

use super::core::Move;
use crate::core::bitboard::Board;

pub type SearchMetadataCallback = Box<dyn Fn(SearchMetadata) -> ()>;
pub type LogCallback = Box<dyn Fn(&str) -> ()>;
pub type GetSystemTimeCallback = Box<dyn Fn() -> Duration>;

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

pub fn from_name(
    name: &str, board: &Board, 
    update_metadata_callback: SearchMetadataCallback, 
    info_callback: LogCallback, 
    should_abort_callback: ShouldAbortSearchCallback, 
    get_system_time_callback: GetSystemTimeCallback
) -> Box<dyn Engine> {
    return match name.to_lowercase().as_str() {
            ab_engine::ENGINE_NAME => Box::new(StandardAlphaBetaEngine::new(board, update_metadata_callback, info_callback, should_abort_callback, get_system_time_callback)),
            random_engine::ENGINE_NAME => Box::new(RandomEngine::new(board, update_metadata_callback, info_callback, should_abort_callback)),
            _ => panic!("Attempted to instantiate engine with invalid name: {}", name)
    };
}