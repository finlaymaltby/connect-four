use crate::basic::*;
use crate::board::{Board, CloneBoard, MutBoard, bit_col};
use std::hash::Hash;

/// A board implementation using bit manipulation for storage with
/// customised equality and hashing for symmetry.
/// Each column is stored as a BitCol.
#[derive(Clone, Debug)]
pub struct SymmBoard {
    cols: [bit_col::BitCol; column::COUNT],
}
impl Board for SymmBoard {
    const EMPTY: Self = SymmBoard {
        cols: [bit_col::BitCol::EMPTY; column::COUNT],
    };

    fn get(&self, cell: &Cell) -> Option<Token> {
        self.cols[usize::from(cell.col)].get(&cell.row)
    }

    fn can_place(&self, col: &column::Idx) -> bool {
        !self.cols[usize::from(*col)].is_full()
    }

    fn place(&mut self, col: &column::Idx, token: &Token) -> Option<Cell> {
        if self.can_place(col) == false {
            return None;
        }

        let col_idx = usize::from(*col);
        self.cols[col_idx].force_push(token);
        Some(Cell {
            col: *col,
            row: row::Idx::try_from(self.cols[col_idx].count() - 1).unwrap(),
        })
    }
}

impl CloneBoard for SymmBoard {}

impl MutBoard for SymmBoard {
    fn unplace(&mut self, cell: &Cell) {
        self.cols[usize::from(cell.col)].force_pop();
    }
}

impl PartialEq for SymmBoard {
    fn eq(&self, other: &Self) -> bool {
        self.cols == other.cols
            || self
                .cols
                .iter()
                .zip(other.cols.iter().rev())
                .all(|(a, b)| a == b)
    }
}

impl Eq for SymmBoard {}

impl Hash for SymmBoard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cols[3].hash(state);
        for i in 0..3 {
            let col_a = self.cols[i];
            let col_b = self.cols[6 - i];
            state.write_u8(col_a.as_u8() | col_b.as_u8());
            state.write_u8(col_a.as_u8() & col_b.as_u8());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::DefaultHasher;
    use std::hash::Hasher;

    use super::*;

    make_board_tests!(SymmBoard);
    make_mut_board_tests!(SymmBoard);

    #[test]
    fn test_symmetry() {
        let mut board_a = SymmBoard::EMPTY;
        let mut board_b = SymmBoard::EMPTY;
        let mut token = Token::START;
        for _ in row::BOTTOM_UP {
            for col in column::IDXS {
                board_a.place(&col, &token).unwrap();
                board_b.place(&col.flipped(), &token).unwrap();

                assert_eq!(board_a, board_b, "Symmetric SymmBoards are not equal");
                let mut hasher_a = DefaultHasher::new();
                let mut hasher_b = DefaultHasher::new();
                board_a.hash(&mut hasher_a);
                board_b.hash(&mut hasher_b);
                assert_eq!(
                    hasher_a.finish(),
                    hasher_b.finish(),
                    "Symmetric SymmBoards have different hashes"
                );
            }
        }
    }
}
