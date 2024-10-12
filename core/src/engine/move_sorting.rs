use crate::core::{bitboard::{constants::WHITE, Board}, move_list::MoveList, Color, Move};

use super::pv::PrincipalVariation;

fn eval_move(board: &Board, mv: &Move) -> i32 {
    if mv.is_quiet() {
        // If move is quiet, give -MAX
        return i32::MIN;
    }
    else {
        // If move is capture, give CAPTURED - TAKING
        let from_piece = board.get_piece(mv.from);
        return mv.captured.eval_score() - from_piece.eval_score();
    }
}

fn eval_move_with_pv(board: &Board, mv: &Move, pv_move: &Move) -> i32 {
    if mv == pv_move {
        return i32::MAX;
    }
    return eval_move(board, mv);
}

pub fn sort_moves_simple(board: &Board, move_list: &mut MoveList, depth: usize, previous_pv: &mut Vec<Move>) {
    let mut moves = move_list.get_underlying_vec();
    let mut move_scores = if previous_pv.len() >= depth as usize {
        let pv_move = previous_pv.pop().unwrap();
        moves.iter().map(|mv| eval_move_with_pv(board, mv, &pv_move)).collect()
    }
    else {
        moves.iter().map(|mv| eval_move(board, mv)).collect()
    };
    insertion_sort(&mut move_scores, &mut moves);
}

fn insertion_sort(list_keys: &mut Vec<i32>, list_values: &mut Vec<Move>) {
    for i in 1..list_keys.len() {
        let mut j = i;
        let key = list_keys[i];
        let value = list_values[i]; 

        while j > 0 && list_keys[j - 1] < key {
            list_keys[j] = list_keys[j - 1];
            list_values[j] = list_values[j - 1];
            j -= 1;
        }
        list_keys[j] = key;
        list_values[j] = value;
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::core::bitboard::Board;
    use crate::core::move_list::MoveList;
    use crate::core::{Move, Piece};
    use crate::core::tests::BOARD_CONSTANT_STATE;
    use crate::engine::move_sorting::sort_moves_simple;

    use super::insertion_sort;

    #[test]
    fn test_move_sort() {
        let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
        let board = Board::new(constant_state);
        assert_eq!("a2a3", Move::to_algebraic(&Move::from_algebraic(&board, "a2a3")));

        // Test insertion sort
        let mut moves = vec![Move::new(&board, 0, 0, Piece::Empty, Piece::Empty); 5];
        let mut scores = vec![5, 3, 1, 2, 4];
        insertion_sort(&mut scores, &mut moves);
        assert_eq!(scores, vec![5,4,3,2,1]);

        // Test move sorting
        let board = Board::from_fen("7k/8/8/8/8/P1Q5/1b6/2R4K b - - 0 1", Rc::new(BOARD_CONSTANT_STATE.clone()));
        let mut moves = vec![
            Move::from_algebraic(&board, "b2a3"), // Pawn capture
            Move::from_algebraic(&board, "b2c3"), // Queen capture
            Move::from_algebraic(&board, "b2c1"), // Rook capture
            Move::from_algebraic(&board, "b2a1") // Quiet capture
        ];
        let mut move_list = MoveList::from_vec(moves);
        sort_moves_simple(&board, &mut move_list, 3, &mut Vec::new());
        let move_list_algebraic: Vec<String> = move_list.to_vec().iter().map(|mv| mv.to_algebraic()).collect();
        assert_eq!(move_list_algebraic, ["b2c3", "b2c1", "b2a3", "b2a1"]);

        // Test move sorting with principal variation
        let board = Board::from_fen("7k/8/8/8/8/P1Q5/1b6/2R4K b - - 0 1", Rc::new(BOARD_CONSTANT_STATE.clone()));
        let moves = vec![
            Move::from_algebraic(&board, "b2a3"), // Pawn capture
            Move::from_algebraic(&board, "b2c3"), // Queen capture
            Move::from_algebraic(&board, "b2c1"), // Rook capture
            Move::from_algebraic(&board, "b2a1") // Quiet capture
        ];
        let mut pv = vec![
            Move::from_algebraic(&board, "b2a3"),
            Move::from_algebraic(&board, "b2c3"), 
            Move::from_algebraic(&board, "b2c1")
        ];
        let mut move_list = MoveList::from_vec(moves);
        sort_moves_simple(&board, &mut move_list, 3, &mut pv);
        
        let move_list_algebraic: Vec<String> = move_list.to_vec().iter().map(|mv| mv.to_algebraic()).collect();
        assert_eq!(move_list_algebraic, ["b2c1", "b2c3", "b2a3", "b2a1"]);
        assert_eq!(pv.len(), 2);

        sort_moves_simple(&board, &mut move_list, 2, &mut pv);
        let move_list_algebraic: Vec<String> = move_list.to_vec().iter().map(|mv| mv.to_algebraic()).collect();
        assert_eq!(move_list_algebraic, ["b2c3", "b2c1", "b2a3", "b2a1"]);
        assert_eq!(pv.len(), 1);

        sort_moves_simple(&board, &mut move_list, 1, &mut pv);
        let move_list_algebraic: Vec<String> = move_list.to_vec().iter().map(|mv| mv.to_algebraic()).collect();
        assert_eq!(move_list_algebraic, ["b2a3", "b2c3", "b2c1", "b2a1"]);
        assert_eq!(pv.len(), 0);

        // No moves left in PV, should just return regular movesort
        sort_moves_simple(&board, &mut move_list, 3, &mut pv);
        let move_list_algebraic: Vec<String> = move_list.to_vec().iter().map(|mv| mv.to_algebraic()).collect();
        assert_eq!(move_list_algebraic, ["b2c3", "b2c1", "b2a3", "b2a1"]);
    }
}