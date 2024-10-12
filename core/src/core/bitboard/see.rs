use std::cmp::max;

use bitboard::{BLACK, COLUMNS, KING_MOVE_MASKS, KNIGHT_MOVE_MASKS, WHITE};

use super::Board;
use crate::core::*;

impl Board {
    #[inline]
    fn see_help_update_occupancies(
        from: u8,
        piece_index: usize,
        occupancy: &mut u64,
        pieces: &mut [u64; 13],
    ) {
        pieces[piece_index] ^= 1u64 << from;
        *occupancy ^= 1u64 << from;
    }

    /// Generate threats by pawns to  ```pos``` assuming that ```COLOR``` is to move
    fn pawn_threats<const COLOR: bool>(
        &self,
        pos: u8,
        occupancy: &mut u64,
        pieces: &mut [u64; 13],
    ) -> u64 {
        let square = 1u64 << pos;
        let index = match COLOR {
            WHITE => Piece::WhitePawn,
            BLACK => Piece::BlackPawn,
        }
        .to_u8() as usize;
        let mask = match COLOR {
            WHITE => ((square << 7) & (!COLUMNS[7])) | ((square << 9) & (!COLUMNS[0])),
            BLACK => ((square >> 9) & (!COLUMNS[7])) | ((square >> 7) & (!COLUMNS[0])),
        };
        let pawns = pieces[index];
        let threat_mask = mask & pawns;
        *occupancy ^= threat_mask;
        pieces[index] ^= threat_mask;
        return threat_mask;
    }

    /// Generates all attacks to ```square``` for both sides
    /// by bishop like pieces. Will only generate moves that
    /// are from squares in occupancy.
    ///
    /// returns a bit set with all squares from which a piece
    /// can move to perform the attack
    fn bishop_like_attacks(&self, square: usize, occupancy: u64) -> u64 {
        let bishop_like = self.piece_sets[Piece::WhiteBishop.to_u8() as usize]
            | self.piece_sets[Piece::BlackBishop.to_u8() as usize]
            | self.piece_sets[Piece::WhiteQueen.to_u8() as usize]
            | self.piece_sets[Piece::BlackQueen.to_u8() as usize];
        let attack_mask = self.runtime_constants.bishop_magic(square, occupancy);
        attack_mask & bishop_like & occupancy
    }

    /// Generates all attacks to ```square``` for both sides
    /// by rook like pieces. Will only generate moves that
    /// are from squares in occupancy.
    ///
    /// returns a bit set with all squares from which a piece
    /// can move to perform the attack
    fn rook_like_attacks(&self, square: usize, occupancy: u64) -> u64 {
        let rook_like = self.piece_sets[Piece::WhiteRook.to_u8() as usize]
            | self.piece_sets[Piece::BlackRook.to_u8() as usize]
            | self.piece_sets[Piece::WhiteQueen.to_u8() as usize]
            | self.piece_sets[Piece::BlackQueen.to_u8() as usize];
        let attack_mask = self.runtime_constants.rook_magic(square, occupancy);
        attack_mask & rook_like & occupancy
    }

    /// Generates all attacks to ```square``` for both sides.
    ///
    /// Will only use occupancy when considering bishop- and
    /// rook-like pieces.
    ///
    /// returns a bit set with all squares from which a piece
    /// can move to perform the attack
    fn all_attacks(&self, square: usize, occupancy: u64) -> u64 {
        let square_mask = 1u64 << square;
        let white_pawns = self.piece_sets[Piece::WhitePawn.to_u8() as usize];
        let black_pawns = self.piece_sets[Piece::BlackPawn.to_u8() as usize];
        let mut attack_set = (((square_mask << 7) & (!COLUMNS[7]))
            | ((square_mask << 9) & (!COLUMNS[0])))
            & white_pawns; //Threats by white_pawns
        attack_set |= (((square_mask >> 9) & (!COLUMNS[7])) | ((square_mask >> 7) & (!COLUMNS[0])))
            & black_pawns; //Threats by black_pawns
                           //knights
        let knights = self.piece_sets[Piece::WhiteKnight.to_u8() as usize]
            | self.piece_sets[Piece::BlackKnight.to_u8() as usize];
        attack_set |= knights & KNIGHT_MOVE_MASKS[square];
        //kings
        let kings = self.piece_sets[Piece::WhiteKing.to_u8() as usize]
            | self.piece_sets[Piece::BlackKing.to_u8() as usize];
        attack_set |= kings & KING_MOVE_MASKS[square];
        //sliding pieces
        attack_set |= self.bishop_like_attacks(square, occupancy);
        attack_set |= self.rook_like_attacks(square, occupancy);
        return attack_set;
    }

    /// Find least valuable attacker in attack set and  its value, if applicable
    fn least_valuable_attacker(
        &self,
        attack_set: u64,
        least_piece: &mut Piece,
        to_move: Color
    ) -> u64 {
        let iter = match to_move {
            Color::White => Piece::white_pieces(),
            Color::Black => Piece::black_pieces(),
        };
        for piece in iter {
            *least_piece = *piece;
            let set = attack_set & self.piece_sets[piece.to_u8() as usize];
            if set > 0 {
                //Lowest value attacker found, extract lowest bit
                return set ^ (set & (set - 1));
            }
        }
        return 0;
    }

    pub fn static_exchange_evaluation(
        &self,
        from_square: u8,
        to_square: u8
    ) -> i32 {
        let mut occupancy = !self.get_piece_set(Piece::Empty);
        let mut possible_attacks = self.all_attacks(to_square as usize, occupancy);
        //At most 32 pieces on the board
        let mut gain = [0i32; 32];
        //Pieces which force update of sliding piece moves
        let reexamine_xrays = self.get_piece_set(Piece::WhitePawn)
            | self.get_piece_set(Piece::WhiteBishop)
            | self.get_piece_set(Piece::WhiteRook)
            | self.get_piece_set(Piece::WhiteQueen)
            | self.get_piece_set(Piece::WhiteKing)
            | self.get_piece_set(Piece::BlackPawn)
            | self.get_piece_set(Piece::BlackBishop)
            | self.get_piece_set(Piece::BlackRook)
            | self.get_piece_set(Piece::BlackQueen)
            | self.get_piece_set(Piece::BlackKing);
        gain[0] = self.mailboard[to_square as usize].eval_score();
        let mut attacking_piece = self.mailboard[from_square as usize];
        let mut from = 1u64 << from_square;
        let mut depth = 1;
        let mut to_move = self.current_player;
        loop {
            //Value of piece currently on square minus square on 
            gain[depth] = attacking_piece.eval_score() - gain[depth - 1]; 
            possible_attacks ^= from;
            occupancy ^= from;
            if from & reexamine_xrays > 0 {
                possible_attacks |= self.bishop_like_attacks(to_square as usize, occupancy);
                possible_attacks |= self.rook_like_attacks(to_square as usize, occupancy);
            }
            to_move = to_move.next_player();
            from = self.least_valuable_attacker(possible_attacks, &mut attacking_piece, to_move);
            if from == 0 {
                break;
            }
            depth += 1;
        }
        depth -= 1;
        while depth > 0 {
            gain[depth - 1] = -max(-gain[depth-1], gain[depth]);
            depth -= 1;
        }
        return gain[0];
    }
}
