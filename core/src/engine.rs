use super::core::Move;
pub mod ab_engine;
use crate::core::bitboard::Board;

pub enum EngineType {
    Standard,
}

pub trait Engine {
    fn make_move(&mut self, mv: &Move);
    fn search(&mut self) -> &Vec<Move>;
    fn get_type(&self, mv: &Move) -> &EngineType;

    fn get_board(&mut self) -> &mut Board;
}