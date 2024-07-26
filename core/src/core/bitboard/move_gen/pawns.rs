use super::Board;
use super::bitboard::constants::*;
use crate::core::*;

const PROMOTION_PIECES_WHITE : [Piece; 4] = [Piece::WhiteQueen, Piece::WhiteRook, Piece::WhiteBishop, Piece::WhiteKnight];
const PROMOTION_PIECES_BLACK : [Piece; 4] = [Piece::BlackQueen, Piece::BlackRook, Piece::BlackBishop, Piece::BlackKnight];


impl Board {
    fn extract_pawn_loop<const FROM_OFFSET: i8, const CAPTURES: bool, const WHITE: bool, const PROMOTION: bool>(&self, mut move_mask: u64, moves : &mut Vec<Move>) {
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            move_mask &= move_mask - 1;
            let taken = match CAPTURES {
                false => Piece::Empty,
                true => self.mailboard[index as usize]
            };
            if PROMOTION {
                let promotion_pieces = match WHITE {
                    true => PROMOTION_PIECES_WHITE,
                    false => PROMOTION_PIECES_BLACK
                };
                for piece in promotion_pieces {
                    moves.push(Move {
                        to: index,
                        from: (index as i8 + FROM_OFFSET) as u8,
                        promotion: piece,
                        captured: taken
                    });
                }
            } else {
                moves.push(Move {
                    to: index,
                    from: (index as i8 + FROM_OFFSET) as u8,
                    promotion: Piece::Empty,
                    captured: taken
                });
            }
        }
    }

    fn extract_pawn_moves<const FROM_OFFSET: i8, const CAPTURES: bool, const WHITE: bool>(&self, move_mask: u64, moves : &mut Vec<Move>) {
        let promotion_mask = match WHITE {
            true => ROWS[0],
            false => ROWS[7]
        };
        self.extract_pawn_loop::<FROM_OFFSET, CAPTURES, WHITE, true>(move_mask & promotion_mask, moves);
        self.extract_pawn_loop::<FROM_OFFSET, CAPTURES, WHITE, false>(move_mask & (!promotion_mask), moves);
    }

    pub fn generate_white_pawn_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let full_occupancy = white_occupancy | black_occupancy;
        let white_pawn_occupancy = self.piece_sets[Piece::WhitePawn.to_u8() as usize];

        // Move forward
        let forward_move_mask = (white_pawn_occupancy >> 8) & !(full_occupancy);
        self.extract_pawn_moves::<8, false, true>(forward_move_mask, moves);

        // Second rank double move
        let double_move_mask = ((forward_move_mask & ROWS[5]) >> 8) & !(full_occupancy);
        self.extract_pawn_loop::<16, false, true, false>(double_move_mask, moves);

        // Add EP square to occupancy as a virtual piece
        let black_occupancy_with_ep = match self.ep {
            0 => black_occupancy,
            _ => black_occupancy | (1 << (self.ep + 16 - 1)) 
        };
        
        // Captures left
        let left_captures_mask = (white_pawn_occupancy >> 9) & !(COLUMNS[7]) & black_occupancy_with_ep;
        self.extract_pawn_moves::<9, true, true>(left_captures_mask, moves);

        // Captures right
        let right_captures_mask = (white_pawn_occupancy >> 7) & !(COLUMNS[0]) & black_occupancy_with_ep;
        self.extract_pawn_moves::<7, true, true>(right_captures_mask, moves);
    }

    pub fn generate_black_pawn_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let full_occupancy = white_occupancy | black_occupancy;
        let black_pawn_occupancy = self.piece_sets[Piece::BlackPawn.to_u8() as usize];

        // Move forward
        let forward_move_mask = (black_pawn_occupancy << 8) & !(full_occupancy);
        self.extract_pawn_moves::<-8, false, false>(forward_move_mask, moves);

        // Second rank double move
        let double_move_mask = ((forward_move_mask & ROWS[2]) << 8) & !(full_occupancy);
        self.extract_pawn_loop::<-16, false, false, false>(double_move_mask, moves);
        
        // Add EP square to occupancy as a virtual piece
        let white_occupancy_with_ep = match self.ep {
            0 => white_occupancy,
            _ => white_occupancy | (1 << (40 + self.ep - 1)) 
        };

        // Captures left
        let left_captures_mask = (black_pawn_occupancy << 9) & !(COLUMNS[0]) & white_occupancy_with_ep;
        self.extract_pawn_moves::<-9, true, false>(left_captures_mask, moves);

        // Captures right
        let right_captures_mask = (black_pawn_occupancy << 7) & !(COLUMNS[7]) & white_occupancy_with_ep;
        self.extract_pawn_moves::<-7, true, false>(right_captures_mask, moves);
    }
}