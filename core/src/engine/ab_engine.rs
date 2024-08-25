
use super::{Engine, SearchMetadataCallback, ShouldAbortSearchCallback};
use crate::core::bitboard::Board;
use crate::core::*;
use move_list::MoveList;
use rand::seq::SliceRandom;
use std::{thread, time::Duration};
use std::time::Instant;


#[allow(unused)]
pub struct StandardAlphaBetaEngine {

}

#[allow(unused)]
impl Engine for StandardAlphaBetaEngine {
    fn search(&mut self, board: &Board, metadata_callback: SearchMetadataCallback, should_abort_search_callback: ShouldAbortSearchCallback) -> Vec<Move> {
        let mut moves = MoveList::empty();
        board.get_moves(&mut moves);
        let pv = vec!(*moves.to_vec().choose(&mut rand::thread_rng()).unwrap());
        metadata_callback(super::SearchMetadata { depth: 2, eval: 7.0, pv: moves.to_vec() });


        let now = Instant::now();

        loop {
            thread::sleep(Duration::from_millis(100));
            
            if (should_abort_search_callback() || now.elapsed().as_millis() > 2000) {
                break;
            }
        }

        return pv;
    }
}

impl StandardAlphaBetaEngine {
    pub fn new() -> StandardAlphaBetaEngine {
        return StandardAlphaBetaEngine {};
    }
}