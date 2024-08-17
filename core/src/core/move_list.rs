use super::{GameStatus, Move, Piece};


const MAX_MOVE_COUNT: usize = 210;


pub enum SearchResult {
    InProgress,
    Stalemate,
    Loss
}

/// MoveList which unsafely wraps a Vector, to avoid runtime checks.
/// This is valid as we can garantuee no chess position has more than MAX_MOVE_COUNT=210 valid moves
pub struct MoveList {
    moves: Vec<Move>,
    result: SearchResult
}

impl MoveList {
    pub fn empty() -> MoveList {
        return MoveList {
            moves: Vec::with_capacity(MAX_MOVE_COUNT),
            result: SearchResult::InProgress
        }
    }

    /// Push a move to the move list. No more than MAX_MOVE_COUNT=210 valid moves can be pushed at once.
    pub fn push(&mut self, mv: Move) {
        unsafe {
            let len = self.moves.len();
            self.moves.set_len(len + 1);
            *self.moves.get_unchecked_mut(len) = mv;
        }
    }

    /// Pop a move from the move list. Popping an empty MoveList leads to undefined behaviour.
    pub fn pop(&mut self) -> Move {
        unsafe {
            let len = self.moves.len() - 1;
            self.moves.set_len(len);
            return *self.moves.get_unchecked_mut(len);

        }
    }

    pub fn clear(&mut self) {
        unsafe { self.moves.set_len(0); }
        self.set_result(SearchResult::InProgress)
    }

    pub fn len(&self) -> usize {
        return self.moves.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Move> {
        return self.moves.iter();
    }

    pub fn to_vec(&self) -> Vec<Move> {
        return self.moves.clone();
    }

    pub fn set_result(&mut self, result: SearchResult) {
        self.result = result;
    }

    pub fn result(&self) -> SearchResult {
        self.result
    }
}