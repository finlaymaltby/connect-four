use crate::basic::{Cell, Token, column, row};
use crate::board::{Board, CloneBoard, MutBoard};
use crate::test_positions::Position;

use paste;

macro_rules! make_test_with_board_on_position {
    ($func:expr, $b:ty, $pos:ident) => {
        paste::paste! {
            #[test]
            fn [< $b:snake _on_ $pos:lower >]() {
                crate::algorithms::testing::assert_output::<$b>(crate::test_positions::$pos, $func);
            }
        }
    };
}

macro_rules! make_easy_tests {
    ($func:expr, $($b:ty),+) => {
        $(
            make_test_with_board_on_position!($func, $b, EASY_0);
            make_test_with_board_on_position!($func, $b, EASY_1);
            make_test_with_board_on_position!($func, $b, EASY_2);
        )+
    };
}

macro_rules! make_medium_tests {
    ($func:expr, $($b:ty),+) => {
        $(
            make_test_with_board_on_position!($func, $b, MEDIUM_0);
            make_test_with_board_on_position!($func, $b, MEDIUM_1);
        )+
    };
}

pub fn assert_output<B: Board>(pos: Position, f: impl Fn(B, usize) -> Option<Token>) {
    let board = B::read(pos.board);
    let winner = f(board, pos.depth);
    assert_eq!(winner, pos.winner, "Incorrect result.");
}
