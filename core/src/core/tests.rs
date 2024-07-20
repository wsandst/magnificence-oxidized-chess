#[cfg(test)]
mod tests {
    use crate::core::bitboard::*;
    use crate::core::*;


    #[test]
    fn test_set_piece() {
        let mut board = Board::new();

        // Ensure that there are no out of bounds problems with edges
        board.set_piece_pos(0, 0, &Piece::WhiteQueen);
        assert_eq!(*board.get_piece(0, 0), Piece::WhiteQueen);
        board.set_piece_pos(0, 7, &Piece::WhiteQueen);
        assert_eq!(*board.get_piece(0, 7), Piece::WhiteQueen);
        board.set_piece_pos(7, 0, &Piece::WhiteQueen);
        assert_eq!(*board.get_piece(7, 0), Piece::WhiteQueen);
        board.set_piece_pos(7, 7, &Piece::WhiteQueen);
        assert_eq!(*board.get_piece(7, 7), Piece::WhiteQueen);

        // Check that overwriting of pieces works as intended
        board.set_piece_pos(0, 0, &Piece::WhiteQueen);
        board.set_piece_pos(0, 0, &Piece::BlackQueen);
        assert_eq!(*board.get_piece(0, 0), Piece::BlackQueen);

        // Check that every piece works as intended
        for piece in Piece::iter() {
            board.set_piece_pos(2, 3, &piece);
            assert_eq!(*board.get_piece(2, 3), piece);
        }
    }

    fn assert_board_equal_to_array_board(board: &Board, array_board: &[Piece; 64]) {
        let mut i = 0;
        for piece in array_board.iter() {
            let x = i % 8;
            let y = i / 8;
            assert_eq!(*board.get_piece(x, y), *piece);
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
    }

    #[test]
    fn test_make_unmake_moves() {
        let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -");

        assert_eq!(*board.get_piece(10 % 8, 10 / 8), Piece::BlackPawn);
        assert_eq!(*board.get_piece(2 % 8, 2 / 8), Piece::Empty);
        let mv = Move {from: 10, to: 2, promotion: Piece::Empty, captured: Piece::Empty};
        board.make_move(&mv);
        println!("{}", board);
        assert_eq!(*board.get_piece(10 % 8, 10 / 8), Piece::Empty);
        assert_eq!(*board.get_piece(2 % 8, 2 / 8), Piece::BlackPawn);
        board.unmake_move(&mv);
        assert_eq!(*board.get_piece(10 % 8, 10 / 8), Piece::BlackPawn);
        assert_eq!(*board.get_piece(2 % 8, 2 / 8), Piece::Empty);

        assert_eq!(*board.get_piece(33 % 8, 33 / 8), Piece::WhiteRook);
        assert_eq!(*board.get_piece(37 % 8, 37 / 8), Piece::BlackPawn);
        let mv = Move {from: 33, to: 37, promotion: Piece::Empty, captured: Piece::BlackPawn};
        board.make_move(&mv);
        println!("{}", board);
        assert_eq!(*board.get_piece(33 % 8, 33 / 8), Piece::Empty);
        assert_eq!(*board.get_piece(37 % 8, 37 / 8), Piece::WhiteRook);
        board.unmake_move(&mv);
        assert_eq!(*board.get_piece(33 % 8, 33 / 8), Piece::WhiteRook);
        assert_eq!(*board.get_piece(37 % 8, 37 / 8), Piece::BlackPawn);
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
}