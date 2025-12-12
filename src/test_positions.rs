use crate::algorithms::minimax_basic::{minimax_copy, minimax_mut};
use crate::algorithms::minimax_cached::minimax_cached;
use crate::basic::*;
use crate::board::Board;
use crate::board::array_board::ArrayBoard;
use crate::board::bit_board::BitBoard;
use crate::board::moves_board::MovesBoard;
use crate::board::symmetric_bit_board::SymmetricBitBoard;

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_winner(board: &str, winner: Token, depth: usize) {
        let array_board = ArrayBoard::read(board);
        let moves_board = MovesBoard::read(board);
        let bit_board = BitBoard::read(board);
        let symmetric_bit_board = SymmetricBitBoard::read(board);

        let curr = array_board.curr_player();
        assert_eq!(array_board.curr_player(), moves_board.curr_player());
        assert_eq!(moves_board.curr_player(), bit_board.curr_player());
        assert_eq!(bit_board.curr_player(), symmetric_bit_board.curr_player());

        assert_eq!(minimax_copy(array_board, depth, curr), Some(winner));
        assert_eq!(minimax_copy(moves_board, depth, curr), Some(winner));
        assert_eq!(minimax_copy(bit_board, depth, curr), Some(winner));
        assert_eq!(minimax_copy(symmetric_bit_board, depth, curr), Some(winner));
    }

    #[test]
    fn test0() {
        let board = "|       |
                     |       |
                     |       |
                     |       |
                     |RRRR   |
                     |YYYY   |";
        let winner = Token::Yellow;
        verify_winner(board, winner, 1);
    }

    #[test]
    fn test1() {}
}
