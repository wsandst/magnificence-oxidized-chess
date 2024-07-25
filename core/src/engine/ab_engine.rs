
use super::{Engine, SearchMetadataCallback};
use crate::core::bitboard::Board;
use crate::core::*;
use rand::seq::SliceRandom;

#[allow(unused)]
pub struct StandardAlphaBetaEngine {

}

#[allow(unused)]
impl Engine for StandardAlphaBetaEngine {
    fn search(&mut self, board: &Board, metadata_callback: SearchMetadataCallback) -> Vec<Move> {
        let mut moves = Vec::new();
        board.get_moves(&mut moves);
        let pv = vec!(*moves.choose(&mut rand::thread_rng()).unwrap());
        metadata_callback(super::SearchMetadata { depth: 2, eval: 7.0, pv: moves });
        return pv;
    }
}

impl StandardAlphaBetaEngine {
    pub fn new() -> StandardAlphaBetaEngine {
        return StandardAlphaBetaEngine {};
    }
}