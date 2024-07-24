#[cfg(test)]
mod tests {
    use crate::core::bitboard::*;
    use crate::core::*;
    use strum::IntoEnumIterator;


    #[test]
    fn test_set_piece() {
        let mut board = Board::empty();

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
        let board1 = Board::from_fen(STARTING_POS_FEN);
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
        assert_eq!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0", &board1.to_fen());
        assert_eq!(board1.get_current_player(), Color::White);
        board1.validate();

        // Kiwipete
        let board2 = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");
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
        assert_eq!("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w - - 0", &board2.to_fen());
        assert_eq!(board2.get_current_player(), Color::White);
        board2.validate();

        let board3 = Board::from_fen("8/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/8 b KQkq - 23");
        assert_eq!(board3.get_current_player(), Color::Black);
        assert_eq!("8/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/8 b - - 23", &board3.to_fen());
    }

    #[test]
    fn test_algebraic_notation() {
        let board = Board::from_fen(STARTING_POS_FEN);
        assert_eq!("a2a3", Move::to_algebraic(&Move::from_algebraic(&board, "a2a3")));
        assert_eq!("d4d5", Move::to_algebraic(&Move::from_algebraic(&board, "d4d5")));
        assert_eq!("d8d1", Move::to_algebraic(&Move::from_algebraic(&board, "d8d1")));
        assert_eq!("g2g1q", Move::to_algebraic(&Move::from_algebraic(&board, "g2g1q")));
        assert_eq!("h2h1Q", Move::to_algebraic(&Move::from_algebraic(&board, "h2h1Q")));
        assert_eq!("a2a1b", Move::to_algebraic(&Move::from_algebraic(&board, "a2a1b")));
    }

    #[test]
    fn test_make_unmake_moves() {
        let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -");

        assert_eq!(board.get_piece_pos(10 % 8, 10 / 8), Piece::BlackPawn);
        assert_eq!(board.get_piece_pos(2 % 8, 2 / 8), Piece::Empty);
        let mv = Move {from: 10, to: 2, promotion: Piece::Empty, captured: Piece::Empty};
        board.make_move(&mv);
        println!("{}", board);
        assert_eq!(board.get_piece_pos(10 % 8, 10 / 8), Piece::Empty);
        assert_eq!(board.get_piece_pos(2 % 8, 2 / 8), Piece::BlackPawn);
        board.unmake_move(&mv);
        assert_eq!(board.get_piece_pos(10 % 8, 10 / 8), Piece::BlackPawn);
        assert_eq!(board.get_piece_pos(2 % 8, 2 / 8), Piece::Empty);

        assert_eq!(board.get_piece_pos(33 % 8, 33 / 8), Piece::WhiteRook);
        assert_eq!(board.get_piece_pos(37 % 8, 37 / 8), Piece::BlackPawn);
        let mv = Move {from: 33, to: 37, promotion: Piece::Empty, captured: Piece::BlackPawn};
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
        // Castling
        let mut board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");
        // Black left side
        let mv = Move {from: 4, to: 2, promotion: Piece::Empty, captured: Piece::Empty};
        board.make_move(&mv);
        assert_eq!(board.get_piece_pos(4, 0), Piece::Empty);
        assert_eq!(board.get_piece_pos(3, 0), Piece::BlackRook);
        assert_eq!(board.get_piece_pos(2, 0), Piece::BlackKing);
        assert_eq!(board.get_piece_pos(0, 0), Piece::Empty);
        board.validate();
        board.unmake_move(&mv);
        assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

        // Black right side
        board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");
        let mv = Move {from: 4, to: 6, promotion: Piece::Empty, captured: Piece::Empty};
        board.make_move(&mv);
        assert_eq!(board.get_piece_pos(4, 0), Piece::Empty);
        assert_eq!(board.get_piece_pos(5, 0), Piece::BlackRook);
        assert_eq!(board.get_piece_pos(6, 0), Piece::BlackKing);
        assert_eq!(board.get_piece_pos(7, 0), Piece::Empty);
        board.validate();
        board.unmake_move(&mv);
        assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

        // White left side
        board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");
        let mv = Move {from: 60, to: 62, promotion: Piece::Empty, captured: Piece::Empty};
        board.make_move(&mv);
        assert_eq!(board.get_piece_pos(4, 7), Piece::Empty);
        assert_eq!(board.get_piece_pos(5, 7), Piece::WhiteRook);
        assert_eq!(board.get_piece_pos(6, 7), Piece::WhiteKing);
        assert_eq!(board.get_piece_pos(7, 7), Piece::Empty);
        board.validate();
        board.unmake_move(&mv);
        assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

        // White right side
        board = Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");
        let mv = Move {from: 60, to: 58, promotion: Piece::Empty, captured: Piece::Empty};
        board.make_move(&mv);
        assert_eq!(board.get_piece_pos(4, 7), Piece::Empty);
        assert_eq!(board.get_piece_pos(3, 7), Piece::WhiteRook);
        assert_eq!(board.get_piece_pos(2, 7), Piece::WhiteKing);
        assert_eq!(board.get_piece_pos(0, 7), Piece::Empty);
        board.validate();
        board.unmake_move(&mv);
        assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R");

        // Promotions
        board = Board::from_fen("1r6/P7/8/8/8/8/p7/1R6 b");
        let mv = Move {from: 48, to: 56, promotion: Piece::BlackQueen, captured: Piece::Empty};
        board.make_move(&mv);
        assert_eq!(board.get_piece_pos(0, 6), Piece::Empty);
        assert_eq!(board.get_piece_pos(0, 7), Piece::BlackQueen);
        board.validate();
        println!("{}", board);
        board.unmake_move(&mv);
        assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "1r6/P7/8/8/8/8/p7/1R6");

        board = Board::from_fen("1r6/P7/8/8/8/8/p7/1R6");
        let mv = Move {from: 8, to: 0, promotion: Piece::WhiteBishop, captured: Piece::Empty};
        board.make_move(&mv);
        assert_eq!(board.get_piece_pos(0, 1), Piece::Empty);
        assert_eq!(board.get_piece_pos(0, 0), Piece::WhiteBishop);
        board.validate();
        board.unmake_move(&mv);
        assert_eq!(board.to_fen().split(" ").nth(0).unwrap(), "1r6/P7/8/8/8/8/p7/1R6");
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

    fn assert_move_eq_algebraic(lhs: &Vec<Move>, rhs: &Vec<&str>) {
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

    #[test]
    fn test_pawn_move_gen() {
        let board = Board::new();
        let (white_occupancy, black_occupancy) = board.get_occupancy();
        let mut moves = Vec::new();
        board.generate_white_pawn_moves(&mut moves, white_occupancy, black_occupancy);

        assert_move_eq_algebraic(&moves, &vec!["a2a3","b2b3","c2c3","d2d3","e2e3","f2f3","g2g3","h2h3",
                                               "a2a4","b2b4","c2c4","d2d4","e2e4","f2f4","g2g4","h2h4"]);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, white_occupancy, black_occupancy);
        assert_move_eq_algebraic(&moves, &vec!["a7a6","b7b6","c7c6","d7d6","e7e6","f7f6","g7g6","h7h6",
                                               "a7a5","b7b5","c7c5","d7d5","e7e5","f7f5","g7g5","h7h5"]);

        let board = Board::from_fen("r1bqkbnr/1P2pppp/5P2/2p3P1/1p5P/p7/PPPP2p1/RNBQKB1R");
        let (white_occupancy, black_occupancy) = board.get_occupancy();
        moves.clear();
        board.generate_white_pawn_moves(&mut moves, white_occupancy, black_occupancy);

        assert_move_eq_algebraic(&moves, &vec![
            "b2b3","c2c3","d2d3","g5g6","h4h5",
            "c2c4", "d2d4",
            "b2a3", "f6e7", "f6g7",
            "b7a8Q", "b7b8Q", "b7c8Q", "b7a8R", "b7b8R", "b7c8R", "b7a8B", "b7b8B", "b7c8B", "b7a8N", "b7b8N", "b7c8N"
        ]);
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, white_occupancy, black_occupancy);
        assert_move_eq_algebraic(&moves, &vec![
            "h7h6","g7g6","e7e6","c5c4","b4b3",
            "h7h5", "e7e5",
            "a3b2", "e7f6", "g7f6",
            "g2f1q", "g2g1q", "g2h1q", "g2f1r", "g2g1r", "g2h1r", "g2f1b", "g2g1b", "g2h1b", "g2f1n", "g2g1n", "g2h1n"
        ]);   


        let board = Board::from_fen("8/8/1p6/p1p5/8/P1P5/1P6/8 b - - 0 1");
        let (white_occupancy, black_occupancy) = board.get_occupancy();
        moves.clear();
        board.generate_black_pawn_moves(&mut moves, white_occupancy, black_occupancy);
        assert_move_eq_algebraic(&moves, &vec![
            "a5a4", "b6b5", "c5c4",
        ]);   
    }
}