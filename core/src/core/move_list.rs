use super::{Move, Piece};


pub struct MoveList {
    moves: [Move; 180],
    current_index: u8
}


impl MoveList {
    pub fn empty() -> MoveList {
        return MoveList {
            moves: [Move { from: 0, to: 0, promotion: Piece::Empty, captured: Piece::Empty }; 180],
            current_index: 0
        }
    }

    pub fn to_vec(&self) -> Vec<Move> {
        return self.moves[0..self.current_index as usize].to_vec();
    }

    pub fn push(&mut self, mv: Move) {
        unsafe { *self.moves.get_unchecked_mut(self.current_index as usize) = mv };
        self.current_index += 1;
    }

    pub fn pop(&mut self) -> Move {
        self.current_index -= 1;
        return unsafe { *self.moves.get_unchecked(self.current_index as usize) };
    }

    pub fn len(&self) -> usize {
        return self.current_index as usize;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Move> {
        return self.moves[0..self.current_index as usize].iter();
    }

    pub fn clear(&mut self) {
        self.current_index = 0;
    }
}