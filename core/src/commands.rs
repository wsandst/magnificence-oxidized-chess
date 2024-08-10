use crate::core::Move;
use super::core::bitboard::*;

pub fn perft(depth: usize, board: &mut Board, reserved_moves: &mut Vec<Vec<Move>>) -> usize {
    let mut moves = match reserved_moves.pop() {
        None => Vec::new(),
        Some(vec) => vec
    };
    moves.clear();
    board.get_moves(&mut moves);
    if depth == 1 {
        let move_count = moves.len();
        reserved_moves.push(moves);
        return move_count;
    }
    let mut total_move_count = 0;
    for mv in moves.iter() {
        board.make_move(&mv);
        total_move_count += perft(depth - 1, board, reserved_moves);
        board.unmake_move(&mv);
    }
    reserved_moves.push(moves);
    return total_move_count;
}

pub fn divide(depth: usize, board: &mut Board, reserved_moves: &mut Vec<Vec<Move>>) -> usize {
    if depth == 1 {
        return perft(1, board, reserved_moves);
    }
    let mut moves = match reserved_moves.pop() {
        None => Vec::new(),
        Some(vec) => vec
    };
    moves.clear();
    board.get_moves(&mut moves);
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
        let mv = Move::from_algebraic(board, mv_algebraic);
        board_copy.make_move(&mv);
    }
    return board_copy;
}