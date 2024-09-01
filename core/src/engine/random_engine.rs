
use super::{Engine, LogCallback, SearchMetadataCallback, ShouldAbortSearchCallback};
use crate::core::bitboard::Board;
use crate::core::*;
use move_list::MoveList;
use rand::seq::SliceRandom;

pub const ENGINE_NAME: &str = "random";

#[allow(unused)]
pub struct RandomEngine {
    update_metadata: SearchMetadataCallback,
    info: LogCallback,
    should_abort_search: ShouldAbortSearchCallback
}

#[allow(unused)]
impl Engine for RandomEngine {
    fn search(&mut self, board: &Board) -> Vec<Move> {
        let mut moves = MoveList::empty();
        board.get_moves(&mut moves);
        let pv = vec!(*moves.to_vec().choose(&mut rand::thread_rng()).unwrap());
        return pv;
    }

    fn get_name(&self) -> &'static str {
        return ENGINE_NAME;
    }
}

impl RandomEngine {
    pub fn new(update_metadata_callback: SearchMetadataCallback, info_callback: LogCallback, should_abort_callback: ShouldAbortSearchCallback) -> RandomEngine {
        return RandomEngine {
            update_metadata: update_metadata_callback,
            info: info_callback,
            should_abort_search: should_abort_callback
        };
    }
}