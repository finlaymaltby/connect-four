#![feature(step_trait, new_range_api)]


use crate::algorithms::minimax_basic::{minimax_copy, minimax_mut};
use crate::algorithms::minimax_cached::minimax_cached;
use crate::basic::*;
use crate::board::Board;
use crate::board::array_board::ArrayBoard;
use crate::board::moves_board::MovesBoard;
use crate::board::bit_board::BitBoard;
use crate::board::symmetric_bit_board::SymmetricBitBoard;


#[cfg(test)]
mod tests {
    use crate::board::symmetric_bit_board;

    use super::*;

    fn test_position(board: &str, winner: Token) {
        let mut array_board = ArrayBoard::read(board);
        let moves_board = MovesBoard::read(board);
        let bit_board = BitBoard::read(board);
        let symmetric_bit_board = SymmetricBitBoard::read(board);

        let curr = array_board.curr_player();
        assert_eq!(array_board.curr_player(), moves_board.curr_player());
        assert_eq!(moves_board.curr_player(), bit_board.curr_player());
        assert_eq!(bit_board.curr_player(), symmetric_bit_board.curr_player());


        assert_eq!(minimax_copy(array_board, 8, curr), Some(winner));
        assert_eq!(minimax_copy(moves_board, 8, curr), Some(winner));
        assert_eq!(minimax_copy(bit_board, 8, curr), Some(winner));
        assert_eq!(minimax_copy(symmetric_bit_board, 8, curr), Some(winner));
    }

    #[test]
    fn test1() {
        let board = "|       |
                     |       |
                     |   R R |
                     |  RY Y |
                     |  YR Y |
                     | RRYYYR|";
        let winner = Token::Yellow;
        test_position(board, winner);
    }

    #[test]
    fn test2() {
        let board = "|    R  |
                     |  Y Y  |
                     |  Y R  |
                     |  R Y  |
                     | RY R  |
                     |YYRYR  |";
        let winner = Token::Yellow;
        test_position(board, winner);
    }
}