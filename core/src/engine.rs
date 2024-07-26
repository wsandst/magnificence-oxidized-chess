use super::core::Move;
pub mod ab_engine;
use crate::core::bitboard::Board;

pub type SearchMetadataCallback = Box<dyn Fn(SearchMetadata) -> ()>;
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
    fn search(&mut self, board: &Board, metadata_callback: SearchMetadataCallback, should_abort_search_callback: ShouldAbortSearchCallback) -> Vec<Move>;
}