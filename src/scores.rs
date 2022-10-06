use crate::prelude::*;

// A score that prefers boards with fewer tiles.
pub fn num_tiles(board: &BoardState) -> f64 {
    let mut num = 0.0;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if board.board[x][y] != 0 {
                num -= 1.0;
            }
        }
    }

    num
}