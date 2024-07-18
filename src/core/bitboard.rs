use crate::core::Move;
use crate::core::Piece;
use lazy_static::lazy_static;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    piece_sets : [u64; 12],
    occupancy : u64,
    white_occupancy : u64,
    black_occupancy : u64,
    ep_history : Vec<u8>,
    castling_history : Vec<u8>,
    ep : u8,
    castling : u8
}

impl Board {
    fn make_move(&mut self, mv: &Move) {
        todo!()
    }

    fn unmake_move(&mut self, mv: &Move) {
        todo!()
    }

    fn set_piece(&mut self, x: u8, y: u8, piece: &Piece) {
        todo!()
    }

    fn get_piece(&self, x: u8, y: u8) -> &Piece {
        todo!()
    }

    // NOTE: Should probably use https://docs.rs/arrayvec/latest/arrayvec/ here in the future 
    fn get_moves(&self) -> Vec<Move> {
        todo!();
    }

    fn from_fen(fen: &str) {
        todo!();
    }

    fn to_fen(&self) -> &str {
        todo!();
    }
}

// Lazy initialize some state at the beginning of the program
lazy_static! {
    pub static ref EXAMPLE_DATA: [u64; 12] = {
        let mut masks = [1,2,3,4,5,6,7,8,9,10,11,12];
        
        return masks;
    };
}
