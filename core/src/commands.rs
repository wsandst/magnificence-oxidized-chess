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
