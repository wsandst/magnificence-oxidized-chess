use crate::core::Move;

pub struct PrincipalVariation {
    pv_table: Vec<Move>,
    max_depth: usize
}

impl PrincipalVariation {
    pub fn new() -> PrincipalVariation {
        return PrincipalVariation { 
            pv_table: Vec::new(), 
            max_depth: 0 
        }
    }

    pub fn set_max_depth(&mut self, depth: usize) {
        self.pv_table.resize(depth * depth, Move::empty());
        self.max_depth = depth;
    }

    pub fn set_best_move(&mut self, depth: usize, mv: Move) {
        let index = (self.max_depth - depth) * self.max_depth;

        // Update the move for the current depth
        self.pv_table[index] = mv;
        if depth == 1 {
            return;
        }

        // Propagate from deeper depth to this depth
        for next_index in 0..depth {
            self.pv_table[index + next_index + 1] = self.pv_table[index + self.max_depth + next_index];
        }
    }

    pub fn get_pv(&self) -> Vec<Move> {
        return self.pv_table[0..self.max_depth].to_vec();
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Move, Piece};
    use super::PrincipalVariation;

    #[test]
    fn test_pv() {
        let mut principal_variation = PrincipalVariation::new();
        principal_variation.set_max_depth(3);
        let mv1 = Move {from: 1, to: 0, captured: Piece::Empty, promotion: Piece::Empty, ep: 0, castling: 0, quiet: 0};
        let mv2 = Move {from: 2, to: 0, captured: Piece::Empty, promotion: Piece::Empty, ep: 0, castling: 0, quiet: 0};
        let mv3 = Move {from: 3, to: 0, captured: Piece::Empty, promotion: Piece::Empty, ep: 0, castling: 0, quiet: 0};
        let mv4 = Move {from: 4, to: 0, captured: Piece::Empty, promotion: Piece::Empty, ep: 0, castling: 0, quiet: 0};
        let mv5 = Move {from: 5, to: 0, captured: Piece::Empty, promotion: Piece::Empty, ep: 0, castling: 0, quiet: 0};

        principal_variation.set_best_move(1, mv3);
        principal_variation.set_best_move(2, mv2);
        principal_variation.set_best_move(3, mv1);
        assert_eq!(vec![mv1, mv2, mv3], principal_variation.get_pv());

        principal_variation.set_best_move(1, mv4);
        principal_variation.set_best_move(1, mv1);
        principal_variation.set_best_move(2, mv5);
        principal_variation.set_best_move(2, mv2);
        principal_variation.set_best_move(3, mv3);
        assert_eq!(vec![mv3, mv2, mv1], principal_variation.get_pv());
    }
}