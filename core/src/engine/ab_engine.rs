
use core::panic;
use std::i32;

use super::{move_sorting, Engine, GetSystemTimeCallback, LogCallback, SearchMetadataCallback, ShouldAbortSearchCallback};
use crate::core::bitboard::Board;
use crate::core::*;
use bitboard::constants::KING_VALUE;
use super::pv::PrincipalVariation;
use move_list::{MoveList, MoveListCollection, SearchResult};
use std::{thread, time::Duration};
use std::time::Instant;

use crate::core::bitboard::constants;

pub const ENGINE_NAME: &str = "magnificence";

#[allow(unused)]
pub struct StandardAlphaBetaEngine {
    update_metadata: SearchMetadataCallback,
    info: LogCallback,
    should_abort_search: ShouldAbortSearchCallback,
    get_system_time: GetSystemTimeCallback,

    board: Board,
    move_lists: MoveListCollection,
    nodes_per_depth: Vec<u64>,
    qsearch_nodes: u64,
    pv_table: PrincipalVariation
}

#[allow(unused)]
impl Engine for StandardAlphaBetaEngine {
    fn search(&mut self, board: &Board) -> Vec<Move> {
        //let now = Instant::now();

        /*let now = Instant::now();
        loop {
            thread::sleep(Duration::from_millis(100));
            
            if (should_abort_search_callback() || now.elapsed().as_millis() > 2000) {
                break;
            }
        }*/

        // TODO: Iterative deepening + timekeeping. Will need to keep track of PVs for that probs
        // How do I handle timekeeping when I can't get the current time on WASM? Maybe do per X nodes instead.
        
        let start_time = (self.get_system_time)();

        self.board = board.clone();
        let mut board_copy = board.clone();
        for i in 0..self.nodes_per_depth.len() {
            self.nodes_per_depth[i] = 0;
        }

        let max_depth = 6;
        let mut pv: Vec<Move> = Vec::new();
        for depth in 0..max_depth {
            self.pv_table.set_max_depth(depth);
            let eval = self.alpha_beta(depth, i32::MIN + 1, i32::MAX, &mut pv);
            pv = self.pv_table.get_pv();
            (self.update_metadata)(super::SearchMetadata { depth: depth as usize, eval: eval as f64, pv: pv.clone() });
        }

        let elapsed = (self.get_system_time)() - start_time;
        //let elapsed = now.elapsed();
        (self.info)(&format!("info search took {:.2?} s", (elapsed.as_secs_f64())));
        self.report_node_counts(max_depth);

        return pv;
    }

    fn get_name(&self) -> &'static str {
        return ENGINE_NAME;
    }
}

impl StandardAlphaBetaEngine {

    /// Evaluate the current position using an alpha beta search. Quiescence Search is ran for the leaf nodes.
    fn alpha_beta(&mut self, depth: usize, mut lower_bound: i32, upper_bound: i32, previous_pv: &mut Vec<Move>) -> i32 {
        self.nodes_per_depth[depth as usize] += 1;
        if depth == 0 {
            return self.qsearch(lower_bound, upper_bound);
        }
        let mut moves = self.move_lists.get_move_list();

        self.board.get_moves(&mut moves, false);
        move_sorting::sort_moves_simple(&self.board, &mut moves, depth, previous_pv);

        let returning = match moves.result() {
            SearchResult::Loss => -KING_VALUE * 8 - depth as i32,
            SearchResult::Stalemate => 0,
            SearchResult::InProgress => {
                for mv in moves.iter() {
                    self.board.make_move(mv);
                    let result = -self.alpha_beta(depth - 1, -upper_bound, -lower_bound, previous_pv);
                    self.board.unmake_move(mv);
                    if result > lower_bound {
                        self.pv_table.set_best_move(depth, *mv);
                        lower_bound = result;
                        if lower_bound >= upper_bound {
                            break;
                        }
                    }
                }
                lower_bound
            }
        };
        self.move_lists.push_move_list(moves);
        return returning;
    }

    // Evaluate the current position until it is quiet (no capturing moves).
    pub fn qsearch(&mut self, mut lower_bound: i32, upper_bound: i32) -> i32 {
        // Handle standing pat
        self.nodes_per_depth[0] += 1;
        let eval = self.board.eval();
        if eval > lower_bound {
            lower_bound = eval;
            if eval >= upper_bound {
                return upper_bound;
            }
        }

        let mut moves = self.move_lists.get_move_list();
        self.board.get_moves(&mut moves, true); // Only generate captures

        let returning = match moves.result() {
            SearchResult::Loss => -KING_VALUE * 8,
            SearchResult::Stalemate => 0,
            SearchResult::InProgress => {
                for mv in moves.iter() {
                    self.board.make_move(mv);
                    let result = -self.qsearch(-upper_bound, -lower_bound);
                    self.board.unmake_move(mv);
                    if result > lower_bound {
                        lower_bound = result;
                        if lower_bound >= upper_bound {
                            break;
                        }
                    }
                }
                lower_bound
            }
        };

        self.move_lists.push_move_list(moves);
        return returning;
    }

    fn report_node_counts(&self, depth: usize) {
        let total_nodes: u64 = self.nodes_per_depth.iter().sum();
        for i in 1..depth {
            let nodes = self.nodes_per_depth[depth - i];
            let node_percentage_of_total = (nodes as f64 / total_nodes as f64) * 100.0;
            if (depth - i) == 0 {
                (self.info)(&format!("info depth {}: {} nodes ({:.2}% of nodes)", i, nodes, node_percentage_of_total));
            }
            else {
                let branching_factor = nodes as f64 / self.nodes_per_depth[depth - i + 1] as f64;
                (self.info)(&format!("info depth {}: {} nodes (avg branching factor {:.2}, {:.2}% of nodes)", i, nodes, branching_factor, node_percentage_of_total));
            }
        }
        let node_percentage_of_total = (self.nodes_per_depth[0] as f64 / total_nodes as f64) * 100.0;
        (self.info)(&format!("info qsearch: {} nodes ({:.2}% of nodes)", self.nodes_per_depth[0], node_percentage_of_total));
    }

    pub fn new(
        board: &Board, 
        update_metadata_callback: SearchMetadataCallback, 
        info_callback: LogCallback, 
        should_abort_callback: ShouldAbortSearchCallback,
        get_system_time_callback: GetSystemTimeCallback
    ) -> StandardAlphaBetaEngine {
        return StandardAlphaBetaEngine {
            update_metadata: update_metadata_callback,
            info: info_callback,
            should_abort_search: should_abort_callback,
            get_system_time: get_system_time_callback,
            board: board.clone(),
            move_lists: MoveListCollection::new(),
            nodes_per_depth: vec![0; 100],
            qsearch_nodes: 0,
            pv_table: PrincipalVariation::new()
        };
    }
}