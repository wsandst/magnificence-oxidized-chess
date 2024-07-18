
use super::Engine;
use crate::core::bitboard::Board;

struct StandardAlphaBetaEngine {

}

impl Engine for StandardAlphaBetaEngine {
    fn make_move(&mut self, mv: &crate::core::Move) {
        todo!()
    }

    fn search(&mut self) -> &Vec<crate::core::Move> {
        todo!()
    }

    fn get_type(&self, mv: &crate::core::Move) -> &super::EngineType {
        todo!()
    }
    
    fn get_board(&mut self) -> &mut Board {
        todo!()
    }
}