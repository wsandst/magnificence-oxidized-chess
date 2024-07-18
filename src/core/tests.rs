#[cfg(test)]
mod tests {
    use crate::core::bitboard::*;
    use crate::core::*;


    #[test]
    fn test_set_piece() {
        let mut board = Board::new();

        // Ensure that there are no out of bounds problems with edges
        board.set_piece(0, 0, &Piece::WhiteQueen);
        assert_eq!(*board.get_piece(0, 0), Piece::WhiteQueen);
        board.set_piece(0, 7, &Piece::WhiteQueen);
        assert_eq!(*board.get_piece(0, 7), Piece::WhiteQueen);
        board.set_piece(7, 0, &Piece::WhiteQueen);
        assert_eq!(*board.get_piece(7, 0), Piece::WhiteQueen);
        board.set_piece(7, 7, &Piece::WhiteQueen);
        assert_eq!(*board.get_piece(7, 7), Piece::WhiteQueen);

        // Check that overwriting of pieces works as intended
        board.set_piece(0, 0, &Piece::WhiteQueen);
        board.set_piece(0, 0, &Piece::BlackQueen);
        assert_eq!(*board.get_piece(0, 0), Piece::BlackQueen);

        // Check that every piece works as intended
        for piece in Piece::iter() {
            board.set_piece(2, 3, &piece);
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
        let board1 = Board::new_from_fen(STARTING_POS_FEN);
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
        let board2 = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");
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

    
}