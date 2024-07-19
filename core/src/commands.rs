use super::core::bitboard::*;

pub fn perft(depth: usize, board: &mut Board) -> usize {
    if depth == 1 {
        return board.get_moves().len();
    }
    let mut total_move_count = 0;
    for mv in board.get_moves() {
        board.make_move(&mv);
        total_move_count += perft(depth - 1, board);
    }
    return total_move_count;
}
