#![cfg(test)]

use super::Move;
use std::rc::Rc;

use crate::commands::perft;
use crate::core::bitboard::*;
use crate::core::bitboard::constants::*;
use crate::{commands, core::*};
use lazy_static::lazy_static;
use move_list::MoveList;
use strum::IntoEnumIterator;


#[test]
fn test_set_piece() {
    let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
    let mut board = Board::empty(Rc::clone(&constant_state));

    // Ensure that there are no out of bounds problems with edges
    board.set_piece_pos(0, 0, &Piece::WhiteQueen);
    assert_eq!(board.get_piece_pos(0, 0), Piece::WhiteQueen);
    board.validate();
    board.set_piece_pos(0, 7, &Piece::WhiteQueen);
    assert_eq!(board.get_piece_pos(0, 7), Piece::WhiteQueen);
    board.validate();
    board.set_piece_pos(7, 0, &Piece::WhiteQueen);
    assert_eq!(board.get_piece_pos(7, 0), Piece::WhiteQueen);
    board.validate();
    board.set_piece_pos(7, 7, &Piece::WhiteQueen);
    assert_eq!(board.get_piece_pos(7, 7), Piece::WhiteQueen);
    board.validate();

    // Check that overwriting of pieces works as intended
    board.set_piece_pos(0, 0, &Piece::WhiteQueen);
    board.validate();
    board.set_piece_pos(0, 0, &Piece::BlackQueen);
    board.validate();
    assert_eq!(board.get_piece_pos(0, 0), Piece::BlackQueen);
    board.validate();

    // Check that every piece works as intended
    for piece in Piece::iter() {
        board.set_piece_pos(2, 3, &piece);
        assert_eq!(board.get_piece_pos(2, 3), piece);
    }
    board.validate();
}

fn assert_board_equal_to_array_board(board: &Board, array_board: &[Piece; 64]) {
    let mut i = 0;
    for piece in array_board.iter() {
        let x = i % 8;
        let y = i / 8;
        assert_eq!(board.get_piece_pos(x, y), *piece);
        i += 1;
    }
}

#[test]
fn test_fen() {
    // Starting position
    let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
    let board1 = Board::from_fen(STARTING_POS_FEN, Rc::clone(&constant_state));
    let expected_pieces1 = [
        Piece::BlackRook, Piece::BlackKnight, Piece::BlackBishop, Piece::BlackQueen, Piece::BlackKing, Piece::BlackBishop, Piece::BlackKnight, Piece::BlackRook,
        Piece::BlackPawn, Piece::BlackPawn,   Piece::BlackPawn,   Piece::BlackPawn,  Piece::BlackPawn, Piece::BlackPawn,   Piece::BlackPawn,   Piece::BlackPawn,
        Piece::Empty,     Piece::Empty,       Piece::Empty,       Piece::Empty,      Piece::Empty,     Piece::Empty,       Piece::Empty,       Piece::Empty,
        Piece::Empty,     Piece::Empty,       Piece::Empty,       Piece::Empty,      Piece::Empty,     Piece::Empty,       Piece::Empty,       Piece::Empty,
        Piece::Empty,     Piece::Empty,       Piece::Empty,       Piece::Empty,      Piece::Empty,     Piece::Empty,       Piece::Empty,       Piece::Empty,
        Piece::Empty,     Piece::Empty,       Piece::Empty,       Piece::Empty,      Piece::Empty,     Piece::Empty,       Piece::Empty,       Piece::Empty,
        Piece::WhitePawn, Piece::WhitePawn,   Piece::WhitePawn,   Piece::WhitePawn,  Piece::WhitePawn, Piece::WhitePawn,   Piece::WhitePawn,   Piece::WhitePawn,
        Piece::WhiteRook, Piece::WhiteKnight, Piece::WhiteBishop, Piece::WhiteQueen, Piece::WhiteKing, Piece::WhiteBishop, Piece::WhiteKnight, Piece::WhiteRook,
    ];
    assert_board_equal_to_array_board(&board1, &expected_pieces1);
    assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &board1.to_fen());
    assert_eq!(board1.get_current_player(), Color::White);

    board1.validate();

    // Kiwipete
    let board2 = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w Kq c6 2 3", Rc::clone(&constant_state));
    let expected_pieces2 = [
        Piece::BlackRook,   Piece::Empty,      Piece::Empty,      Piece::Empty,      Piece::BlackKing, Piece::Empty,      Piece::Empty,      Piece::BlackRook,
        Piece::BlackPawn,   Piece::Empty,      Piece::BlackPawn,  Piece::BlackPawn,  Piece::BlackQueen,Piece::BlackPawn,  Piece::BlackBishop,Piece::Empty,
        Piece::BlackBishop, Piece::BlackKnight,Piece::Empty,      Piece::Empty,      Piece::BlackPawn, Piece::BlackKnight,Piece::BlackPawn,  Piece::Empty,
        Piece::Empty,       Piece::Empty,      Piece::Empty,      Piece::WhitePawn,  Piece::WhiteKnight,Piece::Empty,     Piece::Empty,      Piece::Empty,
        Piece::Empty,       Piece::BlackPawn,  Piece::Empty,      Piece::Empty,      Piece::WhitePawn, Piece::Empty,      Piece::Empty,      Piece::Empty,
        Piece::Empty,       Piece::Empty,      Piece::WhiteKnight,Piece::Empty,      Piece::Empty,     Piece::WhiteQueen, Piece::Empty,      Piece::BlackPawn,
        Piece::WhitePawn,   Piece::WhitePawn,  Piece::WhitePawn,  Piece::WhiteBishop,Piece::WhiteBishop,Piece::WhitePawn,  Piece::WhitePawn, Piece::WhitePawn,
        Piece::WhiteRook,   Piece::Empty,      Piece::Empty,      Piece::Empty,      Piece::WhiteKing, Piece::Empty,      Piece::Empty,      Piece::WhiteRook,
    ];
    assert_board_equal_to_array_board(&board2, &expected_pieces2);
    assert_eq!("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w Kq c6 2 3", &board2.to_fen());
    assert_eq!(board2.get_current_player(), Color::White);
    assert_eq!(board2.get_ep(), 3);
    assert_eq!(board2.get_quiet_moves(), 2);
    assert_eq!(board2.get_half_moves(), 3);
    board2.validate();

    let board3 = Board::from_fen("8/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/8 b - d3 23 26", Rc::clone(&constant_state));
    assert_eq!(board3.get_current_player(), Color::Black);
    assert_eq!(board3.get_ep(), 4);
    assert_eq!("8/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/8 b - d3 23 26", &board3.to_fen());
}

#[test]
fn test_algebraic_notation() {
    let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
    let board = Board::from_fen(STARTING_POS_FEN, Rc::clone(&constant_state));
    assert_eq!("a2a3", Move::to_algebraic(&Move::from_algebraic(&board, "a2a3")));
    assert_eq!("d4d5", Move::to_algebraic(&Move::from_algebraic(&board, "d4d5")));
    assert_eq!("d8d1", Move::to_algebraic(&Move::from_algebraic(&board, "d8d1")));
    assert_eq!("g2g1q", Move::to_algebraic(&Move::from_algebraic(&board, "g2g1q")));
    assert_eq!("h2h1Q", Move::to_algebraic(&Move::from_algebraic(&board, "h2h1Q")));
    assert_eq!("a2a1b", Move::to_algebraic(&Move::from_algebraic(&board, "a2a1b")));
}

#[test]
fn test_make_unmake_moves() {
    let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", Rc::clone(&constant_state));

    assert_eq!(board.get_piece_pos(10 % 8, 10 / 8), Piece::BlackPawn);
    assert_eq!(board.get_piece_pos(2 % 8, 2 / 8), Piece::Empty);
    let mv = Move::new(&board, 10, 2, Piece::Empty, Piece::Empty);
    board.make_move(&mv);
    println!("{}", board);
    assert_eq!(board.get_piece_pos(10 % 8, 10 / 8), Piece::Empty);
    assert_eq!(board.get_piece_pos(2 % 8, 2 / 8), Piece::BlackPawn);
    board.unmake_move(&mv);
    assert_eq!(board.get_piece_pos(10 % 8, 10 / 8), Piece::BlackPawn);
    assert_eq!(board.get_piece_pos(2 % 8, 2 / 8), Piece::Empty);

    assert_eq!(board.get_piece_pos(33 % 8, 33 / 8), Piece::WhiteRook);
    assert_eq!(board.get_piece_pos(37 % 8, 37 / 8), Piece::BlackPawn);
    let mv = Move::new(&board, 33, 37, Piece::Empty, Piece::BlackPawn);
    board.make_move(&mv);
    println!("{}", board);
    assert_eq!(board.get_piece_pos(33 % 8, 33 / 8), Piece::Empty);
    assert_eq!(board.get_piece_pos(37 % 8, 37 / 8), Piece::WhiteRook);
    board.unmake_move(&mv);
    assert_eq!(board.get_piece_pos(33 % 8, 33 / 8), Piece::WhiteRook);
    assert_eq!(board.get_piece_pos(37 % 8, 37 / 8), Piece::BlackPawn);
}

#[test]
fn test_make_unmake_moves_special() {
    let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
    // Castling
    let mut board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R", Rc::clone(&constant_state));
    // Black left side
    let mv = Move::new(&board, 4, 2, Piece::Empty, Piece::Empty);
    board.make_move(&mv);
    assert_eq!(board.get_piece_pos(4, 0), Piece::Empty);
    assert_eq!(board.get_piece_pos(3, 0), Piece::BlackRook);
    assert_eq!(board.get_piece_pos(2, 0), Piece::BlackKing);
    assert_eq!(board.get_piece_pos(0, 0), Piece::Empty);
    board.validate();
    board.unmake_move(&mv);
    assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

    // Black right side
    board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R", Rc::clone(&constant_state));
    let mv = Move::new(&board, 4, 6, Piece::Empty, Piece::Empty);
    board.make_move(&mv);
    assert_eq!(board.get_piece_pos(4, 0), Piece::Empty);
    assert_eq!(board.get_piece_pos(5, 0), Piece::BlackRook);
    assert_eq!(board.get_piece_pos(6, 0), Piece::BlackKing);
    assert_eq!(board.get_piece_pos(7, 0), Piece::Empty);
    board.validate();
    board.unmake_move(&mv);
    assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

    // White left side
    board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R", Rc::clone(&constant_state));
    let mv = Move::new(&board, 60, 62, Piece::Empty, Piece::Empty);
    board.make_move(&mv);
    assert_eq!(board.get_piece_pos(4, 7), Piece::Empty);
    assert_eq!(board.get_piece_pos(5, 7), Piece::WhiteRook);
    assert_eq!(board.get_piece_pos(6, 7), Piece::WhiteKing);
    assert_eq!(board.get_piece_pos(7, 7), Piece::Empty);
    board.validate();
    board.unmake_move(&mv);
    assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

    // White right side
    board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R", Rc::clone(&constant_state));
    let mv = Move::new(&board, 60, 58, Piece::Empty, Piece::Empty);
    board.make_move(&mv);
    assert_eq!(board.get_piece_pos(4, 7), Piece::Empty);
    assert_eq!(board.get_piece_pos(3, 7), Piece::WhiteRook);
    assert_eq!(board.get_piece_pos(2, 7), Piece::WhiteKing);
    assert_eq!(board.get_piece_pos(0, 7), Piece::Empty);
    board.validate();
    board.unmake_move(&mv);
    assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

    // Promotions
    board = Board::from_fen("1r6/P7/8/8/8/8/p7/1R6 b", Rc::clone(&constant_state));
    let mv = Move::new(&board, 48, 56, Piece::BlackQueen, Piece::Empty);
    board.make_move(&mv);
    assert_eq!(board.get_piece_pos(0, 6), Piece::Empty);
    assert_eq!(board.get_piece_pos(0, 7), Piece::BlackQueen);
    board.validate();
    board.unmake_move(&mv);
    assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "1r6/P7/8/8/8/8/p7/1R6");

    board = Board::from_fen("1r6/P7/8/8/8/8/p7/1R6", Rc::clone(&constant_state));
    let mv = Move::new(&board, 8, 0, Piece::WhiteBishop, Piece::Empty);
    board.make_move(&mv);
    assert_eq!(board.get_piece_pos(0, 1), Piece::Empty);
    assert_eq!(board.get_piece_pos(0, 0), Piece::WhiteBishop);
    board.validate();
    board.unmake_move(&mv);
    assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "1r6/P7/8/8/8/8/p7/1R6");

    // En passant
    board = Board::from_fen("8/6p1/8/7P/1p6/8/P7/8 w - - 0 1", Rc::clone(&constant_state));
    let move1 = Move::from_algebraic(&board, "a2a4");
    board.make_move(&move1);
    assert_eq!(board.get_ep(), 1);
    let move2 = &Move::from_algebraic(&board, "b4a3");
    board.make_move(&move2);
    assert_eq!(board.to_fen(), "8/6p1/8/7P/8/p7/8/8 w - - 0 3");
    assert_eq!(board.get_ep(), 0);
    board.unmake_move(&move2);
    assert_eq!(board.get_ep(), 1);
    board.unmake_move(&move1);
    assert_eq!(board.get_ep(), 0);
    assert_eq!(board.to_fen(), "8/6p1/8/7P/1p6/8/P7/8 w - - 0 1");

    board = Board::from_fen("8/6p1/8/7P/1p6/8/P7/8 b - - 0 1", Rc::clone(&constant_state));
    let move1 = Move::from_algebraic(&board, "g7g5");
    board.make_move(&move1);
    assert_eq!(board.get_ep(), 7);
    let move2 = &Move::from_algebraic(&board, "h5g6");
    board.make_move(&move2);
    assert_eq!(board.to_fen(), "8/8/6P1/8/1p6/8/P7/8 b - - 0 3");
    assert_eq!(board.get_ep(), 0);
    board.unmake_move(&move2);
    assert_eq!(board.get_ep(), 7);
    board.unmake_move(&move1);
    assert_eq!(board.get_ep(), 0);
    assert_eq!(board.to_fen(), "8/6p1/8/7P/1p6/8/P7/8 b - - 0 1");
}

#[test]
fn test_bit_twiddling() {
    // Test all possible bits
    for i in 0..64 {
        let mut num = 0u64;
        Board::set_bit(&mut num, i);
        assert_eq!(num, 1u64 << i);
        Board::unset_bit(&mut num, i);
        assert_eq!(num, 0u64);
    }
    let mut num = 0b101010101010;
    Board::set_bit(&mut num, 0);   // 0b101010101011;
    Board::set_bit(&mut num, 2);   // 0b101010101111;
    Board::unset_bit(&mut num, 2); // 0b101010101011;
    Board::unset_bit(&mut num, 1); // 0b101010101001;
    assert_eq!(num, 0b101010101001);
}

pub fn validation_perft(depth: usize, board: &mut Board, reserved_moves: &mut Vec<MoveList>) -> usize {
    let mut moves = match reserved_moves.pop() {
        None => MoveList::empty(),
        Some(moves) => moves
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
        let old_board = board.clone();
        board.make_move(&mv);
        total_move_count += validation_perft(depth - 1, board, reserved_moves);
        board.unmake_move(&mv);
        assert!(old_board == *board && board.get_hashkey() == board.calculate_hash(), "Move: {}\n\n Before make/unmake: {}\nZoobrist: {}\nCalculated zoobrist: {}\n\nAfter make/unmake: {}\nZoobrist: {}\nCalculated zoobrist: {}\n\n", 
                mv, old_board.to_string(), old_board.get_hashkey(), old_board.calculate_hash(), board.to_string(), board.get_hashkey(), board.calculate_hash());
        board.validate();
    }
    reserved_moves.push(moves);
    return total_move_count;
}

#[test]
fn board_validation_with_perft() {
    let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
    let mut board = Board::new(Rc::clone(&constant_state));
    let mut reserved_moves : Vec<MoveList> = (0..15).map(|_| MoveList::empty()).collect();
    validation_perft(4, &mut board, &mut reserved_moves);

    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ", Rc::clone(&constant_state));
    let mut reserved_moves : Vec<MoveList> = (0..15).map(|_| MoveList::empty()).collect();
    validation_perft(4, &mut board, &mut reserved_moves);

    let mut board = Board::from_fen("rnbqkbnr/pppppp2/8/6pp/7P/P7/1PPPPPP1/RNBQKBNR b KQkq - 0 6", Rc::clone(&constant_state));
    let mut reserved_moves : Vec<MoveList> = (0..15).map(|_| MoveList::empty()).collect();
    validation_perft(3, &mut board, &mut reserved_moves);

    // rnbqkbnr/pppppp2/8/6pp/7P/P7/1PPPPPP1/RNBQKBNR b KQkq - 0 6
}

#[test]
fn perft_tests() {
    let constant_state = Rc::new(BOARD_CONSTANT_STATE.clone());
    assert!(commands::perft_tests(Rc::clone(&constant_state), 2_000_000), "Perft tests failed, see print output for more details.");
}            


pub fn assert_moves_eq_algebraic(lhs: &MoveList, rhs: &Vec<&str>) {
    let mut lhs_algebraic : Vec<String> = lhs.iter().map(|mv| mv.to_algebraic()).collect();
    lhs_algebraic.sort();
    let mut rhs_algebraic = rhs.clone();
    rhs_algebraic.sort();

    let mut missing_moves = Vec::new();
    let mut extra_moves = Vec::new();

    // Check for extra generated moves
    for mv1 in lhs_algebraic.iter() {
        if rhs_algebraic.iter().position(|mv2| *mv2 == *mv1).is_none() {
            extra_moves.push(mv1);
        }
    }
    // Check for missing generated moves
    for mv1 in rhs_algebraic.iter() {
        if lhs_algebraic.iter().position(|mv2| *mv2 == *mv1).is_none() {
            missing_moves.push(mv1);
        }
    }
    assert!(missing_moves.len() == 0 && extra_moves.len() == 0, 
        "Move generation did not return the expected moves.\nMissing moves: {:?}\nExtra moves: {:?}", missing_moves, extra_moves);
}

lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref BOARD_CONSTANT_STATE: BitboardRuntimeConstants = {
        BitboardRuntimeConstants::create()
    };
}