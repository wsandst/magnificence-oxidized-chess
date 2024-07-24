
use super::{Engine, SearchMetadataCallback};
use crate::core::bitboard::Board;
use crate::core::*;

#[allow(unused)]
pub struct StandardAlphaBetaEngine {

}

#[allow(unused)]
impl Engine for StandardAlphaBetaEngine {
    fn search(&mut self, board: &Board, metadata_callback: SearchMetadataCallback) -> Vec<Move> {
        let pv = vec![Move { from: 12, to: 20, promotion: Piece::Empty, captured: Piece::Empty}];
        metadata_callback(super::SearchMetadata { depth: 2, eval: 7.0, pv: pv.clone() });
        return pv;
    }
}

impl StandardAlphaBetaEngine {
    pub fn new() -> StandardAlphaBetaEngine {
        return StandardAlphaBetaEngine {};
    }
}