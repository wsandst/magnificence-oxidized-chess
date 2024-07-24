use super::Board;
use super::bitboard::constants::*;
use crate::core::*;

const PROMOTION_PIECES_WHITE : [Piece; 4] = [Piece::WhiteQueen, Piece::WhiteRook, Piece::WhiteBishop, Piece::WhiteKnight];
const PROMOTION_PIECES_BLACK : [Piece; 4] = [Piece::BlackQueen, Piece::BlackRook, Piece::BlackBishop, Piece::BlackKnight];


impl Board {
    pub fn generate_white_pawn_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let full_occupancy = white_occupancy | black_occupancy;
        let white_pawn_occupancy = self.piece_sets[Piece::WhitePawn.to_u8() as usize];

        // Move forward
        let forward_move_mask = (white_pawn_occupancy >> 8) & !(full_occupancy);
        let mut move_mask = forward_move_mask & !ROWS[0];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            moves.push(Move {
                to: index, 
                from: index + 8, 
                promotion: Piece::Empty, 
                captured: Piece::Empty
            });
        }

        // Move forward promotions
        move_mask = forward_move_mask & ROWS[0];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            for piece in PROMOTION_PIECES_WHITE {
                moves.push(Move {
                    to: index, 
                    from: index + 8, 
                    promotion: piece, 
                    captured: Piece::Empty
                });
            }
        }

        // Second rank double move
        let double_move_mask = ((forward_move_mask & ROWS[5]) >> 8) & !(full_occupancy);
        let mut move_mask = double_move_mask;
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            moves.push(Move {
                to: index, 
                from: index + 16, 
                promotion: Piece::Empty, 
                captured: self.mailboard[index as usize]
            });
        }
        
        // Captures left
        let left_captures_mask = (white_pawn_occupancy >> 9) & !(COLUMNS[7]) & black_occupancy;
        move_mask = left_captures_mask & !ROWS[0];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            moves.push(Move {
                to: index, 
                from: index + 9, 
                promotion: Piece::Empty, 
                captured: self.mailboard[index as usize]
            });
        }

        // Captures left promotions
        move_mask = left_captures_mask & ROWS[0];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            for piece in PROMOTION_PIECES_WHITE {
                moves.push(Move {
                    to: index, 
                    from: index + 9, 
                    promotion: piece, 
                    captured: Piece::Empty
                });
            }
        }

        // Captures right
        let right_captures_mask = (white_pawn_occupancy >> 7) & !(COLUMNS[0]) & black_occupancy;
        move_mask = right_captures_mask & !ROWS[0];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            moves.push(Move {
                to: index, 
                from: index + 7, 
                promotion: Piece::Empty, 
                captured: self.mailboard[index as usize]
            });
        }

        // Captures right promotions
        move_mask = right_captures_mask & ROWS[0];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            for piece in PROMOTION_PIECES_WHITE {
                moves.push(Move {
                    to: index, 
                    from: index + 7, 
                    promotion: piece, 
                    captured: Piece::Empty
                });
            }
        }

        // TODO: En passant and king safety
    }

    pub fn generate_black_pawn_moves(&self, moves : &mut Vec<Move>, white_occupancy: u64, black_occupancy: u64) {
        let full_occupancy = white_occupancy | black_occupancy;
        let black_pawn_occupancy = self.piece_sets[Piece::BlackPawn.to_u8() as usize];

        // Move forward
        let forward_move_mask = (black_pawn_occupancy << 8) & !(full_occupancy);
        let mut move_mask = forward_move_mask & !ROWS[7];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            moves.push(Move {
                to: index, 
                from: index - 8, 
                promotion: Piece::Empty, 
                captured: Piece::Empty
            });
        }

        // Move forward promotions
        move_mask = forward_move_mask & ROWS[7];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            for piece in PROMOTION_PIECES_BLACK {
                moves.push(Move {
                    to: index, 
                    from: index - 8, 
                    promotion: piece, 
                    captured: Piece::Empty
                });
            }
        }

        // Second rank double move
        let double_move_mask = ((forward_move_mask & ROWS[2]) << 8) & !(full_occupancy);
        let mut move_mask = double_move_mask;
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            moves.push(Move {
                to: index, 
                from: index - 16, 
                promotion: Piece::Empty, 
                captured: self.mailboard[index as usize]
            });
        }
        
        // Captures left
        let left_captures_mask = (black_pawn_occupancy << 9) & !(COLUMNS[0]) & white_occupancy;
        move_mask = left_captures_mask & !ROWS[7];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            moves.push(Move {
                to: index, 
                from: index - 9, 
                promotion: Piece::Empty, 
                captured: self.mailboard[index as usize]
            });
        }

        // Captures left promotions
        move_mask = left_captures_mask & ROWS[7];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            for piece in PROMOTION_PIECES_BLACK {
                moves.push(Move {
                    to: index, 
                    from: index - 9, 
                    promotion: piece, 
                    captured: Piece::Empty
                });
            }
        }

        // Captures right
        let right_captures_mask = (black_pawn_occupancy << 7) & !(COLUMNS[7]) & white_occupancy;
        move_mask = right_captures_mask & !ROWS[7];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            moves.push(Move {
                to: index, 
                from: index - 7, 
                promotion: Piece::Empty, 
                captured: self.mailboard[index as usize]
            });
        }

        // Captures right promotions
        move_mask = right_captures_mask & ROWS[7];
        while move_mask > 0 {
            let index = move_mask.trailing_zeros() as u8;
            Self::unset_bit(&mut move_mask, index);
            for piece in PROMOTION_PIECES_BLACK {
                moves.push(Move {
                    to: index, 
                    from: index - 7, 
                    promotion: piece, 
                    captured: Piece::Empty
                });
            }
        }

        // TODO: En passant and king safety
    }
}