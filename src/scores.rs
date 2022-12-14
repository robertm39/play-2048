use crate::prelude::*;

// A score that prefers boards with fewer tiles.
pub fn num_tiles(board: &mut BoardState) -> f64 {
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
pub fn num_connected_tiles(board: &mut BoardState) -> f64 {
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
pub fn highest_in_corner_or_edge_score(board: &mut BoardState) -> f64 {
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

// A score that sees if the highest tile is in the corner.
pub fn highest_tile_in_corner_score(board: &mut BoardState) -> f64 {
    let highest_tile = board.get_highest_tile();

    if highest_tile <= 2 {
        return 0.0;
    }

    for x in [0, BOARD_SIZE - 1] {
        for y in [0, BOARD_SIZE - 1] {
            if board.board[x][y] == highest_tile {
                return 1.0;
            }
        }
    }
    0.0
}

// A score that sees if there are tiles equal to the highest tile in the center.
pub fn highest_tile_in_center_score(board: &mut BoardState) -> f64 {
    let highest_tile = board.get_highest_tile();

    // I only care if the highest tile is 32 or more
    if highest_tile <= 4 {
        return 0.0;
    }

    let mut total = 0.0;
    for x in 1..BOARD_SIZE-1 {
        for y in 1..BOARD_SIZE-1 {
            if board.board[x][y] == highest_tile {
                total -= 1.0;
            }
        }
    }
    total
}

// Return the number of tiles with an adjacent tile of the same value.
pub fn adjacent_tiles_score(board: &mut BoardState, emp_non_adj: f64, emp_adj: f64, sur_non_adj: f64, sur_adj: f64) -> f64 {
    let mut adj: [[bool; 4]; 4] =
    [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ];

    let mut emp: [[bool; 4]; 4] =
    [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ];

    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let tile = board.board[x][y];

            // Figure out which tiles have neighbors of the same value
            if tile != 0 {
                if x < BOARD_SIZE - 1 && tile == board.board[x+1][y] {
                    adj[x][y] = true;
                    adj[x+1][y] = true;
                }
                if y < BOARD_SIZE - 1 && tile == board.board[x][y+1] {
                    adj[x][y] = true;
                    adj[x][y+1] = true;
                }
            }


            // Figure out which tiles have empty spots next to them
            if tile == 0 {
                if x < BOARD_SIZE - 1 && board.board[x+1][y] != 0 {
                    emp[x+1][y] = true;
                }
                if x > 0 && board.board[x-1][y] != 0 {
                    emp[x-1][y] = true;
                }
                if y < BOARD_SIZE - 1 && board.board[x][y+1] != 0 {
                    emp[x][y+1] = true;
                }
                if y > 0 && board.board[x][y-1] != 0 {
                    emp[x][y-1] = true;
                }
            }
        }
    }

    let mut score = 0.0;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let is_emp = emp[x][y];
            let has_adj = adj[x][y]; 

            if is_emp && has_adj {
                score += emp_adj;
            } else if is_emp && !has_adj {
                score += emp_non_adj;
            } else if (!is_emp) && has_adj {
                score += sur_adj;
            } else {
                score += sur_non_adj;
            }
        }
    }

    score
}

// pub fn smarter_score(board: &mut BoardState) -> f64 {
//     num_tiles(board) + num_connected_tiles(board) + highest_in_corner_or_edge_score(board)
// }

// // A function multiplied by a weight.
// pub struct WeightedFunc {
//     score_func: fn(&mut BoardState) -> f64,
//     weight: f64,
// }

// impl WeightedFunc {
//     pub fn new(score_func: fn(&mut BoardState) -> f64, weight: f64) -> Self {
//         Self {
//             score_func,
//             weight
//         }
//     }

//     pub fn get_score(&self, board: &mut BoardState) -> f64 {
//         self.weight * (self.score_func)(board)
//     }
// }

// // The sum of many functions.
// pub struct ManyFuncs {
//     funcs: Vec<fn(&BoardState) -> f64>
// }

// impl ManyFuncs {
//     pub fn new(funcs: Vec<fn(&BoardState) -> f64>) -> Self {
//         Self {
//             funcs
//         }
//     }

//     // Return the sum of all the scores
//     pub fn get_score(&self, board: &BoardState) -> f64 {
//         let mut total = 0.0;
//         for func in &self.funcs {
//             total += func(board);
//         }
//         total
//     }
// }