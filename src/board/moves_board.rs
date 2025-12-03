use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard};

/// MovesBoard implementation using a vector of placed tokens.
/// Stores only the moves made, reconstructing the board state as needed.
/// The worst representation I could think of.
#[derive(Debug, Clone)]
pub struct MovesBoard {
    moves: Vec<(column::Idx, Token)>,
}

impl MovesBoard {
    fn count_in_column(&self, col: &column::Idx) -> usize {
        self.moves.iter().filter(|(c, _)| c == col).count()
    }
}

impl Board for MovesBoard {
    const EMPTY: Self = MovesBoard { moves: Vec::new() };

    fn get(&self, pos: &Position) -> Option<Token> {
        let mut col_count = 0;
        for (col, token) in &self.moves {
            if *col == pos.col {
                if col_count == usize::from(pos.row) {
                    return Some(*token);
                }
                col_count += 1;
            }
        }
        None
    }

    fn can_place(&self, col: &column::Idx) -> bool {
        self.count_in_column(col) < row::COUNT
    }

    fn force_place(&mut self, col: &column::Idx, token: &Token) -> Position {
        let row = self.count_in_column(col);
        self.moves.push((*col, *token));
        Position {
            row: row::Idx::try_from(row).unwrap(),
            col: *col,
        }
    }
}

impl CloneBoard for MovesBoard {}

impl MutBoard for MovesBoard {
    fn unplace(&mut self, pos: &Position) {
        let mut col_count = 0;
        for i in 0..self.moves.len() {
            let (col, _) = &self.moves[i];
            if *col == pos.col {
                if col_count == usize::from(pos.row) {
                    self.moves.remove(i);
                    return;
                }
                col_count += 1;
            }
        }
    }
}
