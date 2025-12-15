use crate::basic::{Position, Token, column, row};
use crate::board::{Board, MutBoard};

macro_rules! make_test {
    ($b:ty, $mod:ident, $func:ident) => {
        #[test]
        fn $func() {
            crate::board::testing::$mod::$func::<$b>(stringify!($b));
        }
    };
}

macro_rules! make_board_tests {
    ($b:ty) => {
        make_test!($b, board_tests, empty_is_empty);
        make_test!($b, board_tests, cannot_place_in_full_column);
        make_test!($b, board_tests, won_at_basic);
    };
}

macro_rules! make_mut_board_tests {
    ($b:ty) => {
        make_test!($b, mut_board_tests, place_unplace_eq);
    };
}

pub mod board_tests {
    use super::*;

    pub fn empty_is_empty<B: Board>(name: &str) {
        let empty = B::EMPTY;
        for col in column::IDXS {
            for row in row::BOTTOM_UP {
                assert!(
                    empty.get(&Position { col, row }).is_none(),
                    "`{name}::EMPTY` is not empty at ({col}, {row})."
                );
            }
        }
    }

    pub fn cannot_place_in_full_column<B: Board>(name: &str) {
        let mut board = B::EMPTY;
        let mut curr = Token::START;

        for col in column::IDXS {
            for _ in 0..6 {
                assert!(
                    board.can_place(&col),
                    "`{name}::can_place` returned false on a non-full column."
                );
                assert_eq!(
                    curr,
                    board.curr_player(),
                    "`{name}::curr_player` returned incorrect."
                );
                let pos = board.place(&col, &curr);
                assert!(
                    pos.is_some(),
                    "`{name}::place` returned `None` even though `can_place` is true"
                );
                curr = curr.next();
            }

            assert!(
                !board.can_place(&col),
                "`{name}::can_place returned true on a full column."
            );
        }
    }

    pub fn won_at_basic<B: Board>(name: &str) {
        let board = B::read(
            "|       |
             |       |
             |       |
             |RRR    |
             |YYYY   |",
        );
        assert!(
            board.won_at(&Position {
                col: column::Idx::raw(3),
                row: row::Idx::raw(0)
            }),
            "`{name}::won_at returned false on a winning position."
        );
    }
}

pub mod mut_board_tests {
    use super::*;

    pub fn place_unplace_eq<B: Clone + MutBoard>(name: &str) {
        let mut board = B::EMPTY;
        let mut token = Token::START;
        for _ in 0..row::Idx::COUNT {
            for col in column::IDXS {
                let temp = board.clone();

                let Some(pos) = board.place(&col, &token) else {
                    panic!("`{name}::place returned None on a non-full column.");
                };
                board.unplace(&pos);
                assert_eq!(board, temp, "`{name}::unplace∘{name}::place` != id.");
                board.place(&col, &token);
                token = token.next();
            }
        }
    }
}
