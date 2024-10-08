use constants::BitboardRuntimeConstants;

use crate::core::{move_list::{MoveList, MoveListCollection}, Move};
use super::core::bitboard::*;

const USE_LEAF_NODE_OPTIMIZATION : bool = true;

pub fn perft(depth: usize, board: &mut Board, reserved_moves: &mut MoveListCollection) -> usize {
    if depth <= 0 {
        return 1;
    }
    let mut moves = reserved_moves.get_move_list();
    moves.clear();
    board.get_moves(&mut moves, false);
    if depth == 1 && USE_LEAF_NODE_OPTIMIZATION {
        let move_count = moves.len();
        reserved_moves.push_move_list(moves);
        return move_count;
    }
    let mut total_move_count = 0;
    for mv in moves.iter() {
        board.make_move(&mv);
        total_move_count += perft(depth - 1, board, reserved_moves);
        board.unmake_move(&mv);
    }
    reserved_moves.push_move_list(moves);
    return total_move_count;
}

pub fn divide(depth: usize, board: &mut Board, reserved_moves: &mut MoveListCollection) -> usize {
    let mut moves = reserved_moves.get_move_list();
    moves.clear();
    board.get_moves(&mut moves, false);
    let mut results : Vec<(String, usize)> = Vec::new();
    for mv in moves.iter() {
        board.make_move(&mv);
        let result = perft(depth - 1, board, reserved_moves);
        results.push((mv.to_string(), result));
        board.unmake_move(&mv);
    }
    results.sort();
    for (mv, perft_count) in results.iter() {
        println!("{}: {}", mv, perft_count);
    }
    return results.iter().fold(0, |l, r| l + r.1);
}

pub fn board_from_moves(board: &Board, moves: &Vec<String>) -> Board {
    let mut board_copy = board.clone();
    for mv_algebraic in moves {
        let mv = Move::from_algebraic(&board_copy, mv_algebraic);
        board_copy.make_move(&mv);
    }
    return board_copy;
}

pub fn perft_tests(runtime_constants: std::rc::Rc<BitboardRuntimeConstants>, node_limit: usize) -> bool {
    let tests: Vec<(&str, Vec<u64>)> = vec![
            // These are from chessprogrammingwiki.com
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", vec![20, 400, 8902, 197_281, 4_865_609]),
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ", vec![48, 2039, 97_862, 4_085_603, 193_690_690, 8_031_647_685]),
            ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ", vec![14, 191, 2812, 43_238, 674_624, 11_030_083, 178_633_661, 3_009_794_393]),
            ("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", vec![6, 264, 9_467, 422_333, 15_833_292, 706_045_033]),
            ("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", vec![44, 1_486, 62_379, 2_103_487, 89_941_194]),
            ("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", vec![46, 2_079, 89_890, 3_894_594, 164_075_551, 6_923_051_137]), 
            // Manually added extra case (promotions + ep)
            ("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P2P/2N2Q2/PPPBBPp1/1R2K2R b Kkq h3 0 4", vec![50, 2069, 99_997, 42_35_277])
        ];
    let mut reserved_moves = MoveListCollection::new();
    let mut success = true;
    for (fen, results) in tests {
        let mut board = Board::from_fen(fen, runtime_constants.clone());
        println!("Running fen test on position {fen}");
        for (i, result) in results.iter().enumerate() {
            if *result as usize >= node_limit {
                // Stop searching for positions if the expected node count exceeds the node limit
                break;
            }
            let found = perft(i + 1, &mut board, &mut reserved_moves);
            if found != (*result) as usize {
                println!("Error on depth: {}, expected: {},  found: {}", i + 1, result, found);
                success = false;
                break;
            }
        }
    }
    return success;
}