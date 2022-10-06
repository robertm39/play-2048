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

// A score that likes it when tiles are next to same or similar tiles.
pub fn num_connected_tiles(board: &BoardState) -> f64 {
    let mut score = 0.0;

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let tile = board.board[x][y];

            if tile == 0 {
                continue;
            }

            // Search through all adjacent tiles for a tile that is the same or only one different
            let mut has_same = false;
            let mut has_close = false;
            for diff in [[-1, 0], [1, 0], [0, 1], [0, -1]] {
                let tx = usize::try_from(i32::try_from(x).unwrap() + diff[0]);
                let ty = usize::try_from(i32::try_from(y).unwrap() + diff[1]);

                if let Ok(x0) = tx {
                    if let Ok(y0) = ty {
                        if x0 < BOARD_SIZE && y0 < BOARD_SIZE {
                            let other_tile = board.board[x0][y0];
                            if other_tile == tile {
                                has_same = true;
                                break;
                            } else if other_tile == tile + 1 || other_tile == tile - 1 {
                                has_close = true;
                            }
                        }
                    }
                }
            }

            if has_same {
                score += 0.7;
            } else if has_close {
                score += 0.4;
            }
        }
    }

    score
}

// A score that sees whether the highest tile is in the corner, or maybe on the edge.
pub fn highest_in_corner_score(board: &BoardState) -> f64 {
    let scale = 4.0;

    let mut score = 0.0;

    let mut highest = 1;
    let mut highest_in_corner = false;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let x_ext = x == 0 || x == BOARD_SIZE - 1;
            let y_ext = y == 0 || y == BOARD_SIZE - 1;

            let is_corner = x_ext && y_ext;
            let in_center = !(x_ext || y_ext);

            let tile = board.board[x][y];
            if tile > highest {
                highest_in_corner = false;
                score = 0.0;
                highest = tile;
            }
            if tile >= highest {
                highest_in_corner |= is_corner;
                if in_center {
                    score -= scale;
                }
            }
        }
    }

    if highest_in_corner {
        score += scale;
    }

    if highest <= 2 {
        score = 0.0;
    }

    score
}

pub fn smarter_score(board: &BoardState) -> f64 {
    num_tiles(board) + num_connected_tiles(board) + highest_in_corner_score(board)
}