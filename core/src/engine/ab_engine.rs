
use std::i32;

use super::{Engine, LogCallback, SearchMetadataCallback, ShouldAbortSearchCallback};
use crate::core::bitboard::Board;
use crate::core::*;
use bitboard::constants::KING_VALUE;
use move_list::{MoveList, SearchResult};
use std::{thread, time::Duration};
use std::time::Instant;

use crate::core::bitboard::constants;

pub const ENGINE_NAME: &str = "Magnificence";

#[allow(unused)]
pub struct StandardAlphaBetaEngine {
    update_metadata: SearchMetadataCallback,
    info: LogCallback,
    should_abort_search: ShouldAbortSearchCallback
}

#[allow(unused)]
impl Engine for StandardAlphaBetaEngine {
    fn search(&mut self, board: &Board) -> Vec<Move> {
        /*let now = Instant::now();
        loop {
            thread::sleep(Duration::from_millis(100));
            
            if (should_abort_search_callback() || now.elapsed().as_millis() > 2000) {
                break;
            }
        }*/

        (self.info)("hello world");
        let mut moves = MoveList::empty();
        let mut board_copy = board.clone();
        let mut moves = Vec::new();
        let depth = 4;
        let (eval, mv) = self.alpha_beta(&mut board_copy, &mut moves, depth, i32::MIN + 1, i32::MAX);
        let pv: Vec<Move> = vec!(mv.unwrap());

        (self.update_metadata)(super::SearchMetadata { depth: depth as usize, eval: eval as f64, pv: pv.clone() });
        return pv;
    }

    fn get_name(&self) -> &'static str {
        return ENGINE_NAME;
    }
}

impl StandardAlphaBetaEngine {
    fn alpha_beta(&mut self, board: &mut Board, move_lists: &mut Vec<MoveList>, depth: i32, mut lower_bound: i32, upper_bound: i32) -> (i32, Option<Move>) {
        if depth == 0 {
            return (-board.eval(), None);
        }
        let mut moves = match move_lists.pop() {
            None => MoveList::empty(),
            Some(list) => list
        };
        board.get_moves(&mut moves);
        let mut best_move = None;
        let returning = match moves.result() {
            SearchResult::Loss => -KING_VALUE * 8 - depth,
            SearchResult::Stalemate => 0,
            SearchResult::InProgress => {
                for mv in moves.iter() {
                    board.make_move(mv);
                    let (result, _) = self.alpha_beta(board, move_lists, depth - 1, -upper_bound, -lower_bound);
                    let result = -result;
                    board.unmake_move(mv);
                    if result > lower_bound {
                        lower_bound = result;
                        best_move = Some(*mv);
                        if lower_bound >= upper_bound {
                            break;
                        }
                    }
                }
                lower_bound
            }
        };
        move_lists.push(moves);
        return (returning, best_move);
    }

    pub fn new(update_metadata_callback: SearchMetadataCallback, info_callback: LogCallback, should_abort_callback: ShouldAbortSearchCallback) -> StandardAlphaBetaEngine {
        return StandardAlphaBetaEngine {
            update_metadata: update_metadata_callback,
            info: info_callback,
            should_abort_search: should_abort_callback
        };
    }
}