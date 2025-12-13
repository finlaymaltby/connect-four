use crate::algorithms::minimax_basic::{minimax_copy, minimax_mut};
use crate::algorithms::minimax_cached::minimax_cached;
use crate::basic::*;
use crate::board::Board;
use crate::board::array_board::ArrayBoard;
use crate::board::bit_board::BitBoard;
use crate::board::moves::Moves;
use crate::board::symm_board::SymmBoard;

pub const SIMPLE: (&str, Option<Token>, usize) = (
    "|.......|
     |.......|
     |.......|
     |.......|
     |RRRR...|
     |YYYY...|",
    Some(Token::Yellow),
    1,
);

pub const BOARD0: (&str, Option<Token>, usize) = (
    "|RRY.R.Y|
     |YYR.Y.R|
     |RYR.R.R|
     |YRRYY.Y|
     |RRYRY.Y|
     |YYRYR.Y|",
    Some(Token::Yellow),
    10,
);

pub const BOARD1: (&str, Option<Token>, usize) = (
    "|RRY.R..|
     |YYR.Y..|
     |RYR.R..|
     |YRRYYY.|
     |RRYRYR.|
     |YYRYRYY|",
    Some(Token::Red),
    11,
);

pub const EASY_TEST_BOARDS: [(&str, Option<Token>, usize); 3] = [SIMPLE, BOARD0, BOARD1];

pub const BOARD2: (&str, Option<Token>, usize) = (
    "|.......|
     |Y.Y.R..|
     |R.Y.R..|
     |YRR.Y..|
     |YRYYR..|
     |YYRYR.R|",
    None,
    20,
);

pub const BOARD3: (&str, Option<Token>, usize) = (
    "|.......|
     |..Y.Y..|
     |..Y.R..|
     |..RRY..|
     |.RYRR.Y|
     |YYRYR.R|",
    Some(Token::Red),
    24,
);

pub const MEDIUM_TEST_BOARDS: [(&str, Option<Token>, usize); 5] = [SIMPLE, BOARD0, BOARD1, BOARD2, BOARD3];