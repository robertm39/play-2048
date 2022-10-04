#![warn(clippy::all, clippy::pedantic)]

const BOARD_SIZE: usize = 4;

// The state of the game.
// Contains a 4x4 grid of integers.
// The integer n represents the tile 2^n.
// The integer 0 represents absence of a tile.
// This is enough to hold all tiles obtainable in a normal 2048 game.
#[derive(Debug, Copy, Clone)]
pub struct BoardState {
    board: [[u8;BOARD_SIZE];BOARD_SIZE]
}

impl BoardState {
    pub fn new(b: [[u8;4];4]) -> Self {
        Self {
            board: b
        }
    }
}

// The moves a player can make.
#[derive(Debug)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

// Return the state of the board after sliding the tiles in the specified direction.
// Tested.
pub fn after_slide_tiles(mut gs: BoardState, m: &Move) -> BoardState{
    match m {
        Move::Up => {
            for x in 0..BOARD_SIZE {
                let mut target_y = 0;
                for y in 0..BOARD_SIZE {
                    if gs.board[x][y] != 0 {
                        if target_y != y {
                            // Slide this tile down to the first empty spot
                            gs.board[x][target_y] = gs.board[x][y];
                            gs.board[x][y] = 0;
                        }

                        target_y += 1;
                    }
                }
            }
        }
        Move::Down => {
            for x in 0..BOARD_SIZE {
                let mut target_y = BOARD_SIZE-1;
                for y in (0..BOARD_SIZE).rev() {
                    if gs.board[x][y] != 0 {
                        if target_y != y {
                            // Slide this tile down to the first empty spot
                            gs.board[x][target_y] = gs.board[x][y];
                            gs.board[x][y] = 0;
                        }

                        target_y -= 1;
                    }
                }
            }
        }
        Move::Left => {
            for y in 0..BOARD_SIZE {
                let mut target_x = 0;
                for x in 0..BOARD_SIZE {
                    if gs.board[x][y] != 0 {
                        if target_x != x {
                            // Slide this tile down to the first empty spot
                            gs.board[target_x][y] = gs.board[x][y];
                            gs.board[x][y] = 0;
                        }

                        target_x += 1;
                    }
                }
            }
        }
        Move::Right => {
            for y in 0..BOARD_SIZE {
                let mut target_x = BOARD_SIZE-1;
                for x in (0..BOARD_SIZE).rev() {
                    if gs.board[x][y] != 0 {
                        if target_x != x {
                            // Slide this tile down to the first empty spot
                            gs.board[target_x][y] = gs.board[x][y];
                            gs.board[x][y] = 0;
                        }

                        target_x -= 1;
                    }
                }
            }
        }
    }
    gs
}

// Return the board state after merging tiles in the specified direction.
// Has been tested.
pub fn after_merge_tiles(mut gs: BoardState, m: &Move) -> BoardState {
    match m {
        Move::Up => {
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE-1 {
                    if gs.board[x][y] == 0 {
                        continue;
                    }
                    if gs.board[x][y] == gs.board[x][y+1] {
                        gs.board[x][y+1] = 0;
                        gs.board[x][y] += 1;
                    }
                }
            }
        }
        Move::Down => {
            for x in 0..BOARD_SIZE {
                for y in (1..BOARD_SIZE).rev() {
                    if gs.board[x][y] == 0 {
                        continue;
                    }
                    if gs.board[x][y] == gs.board[x][y-1] {
                        gs.board[x][y-1] = 0;
                        gs.board[x][y] += 1;
                    }
                }
            }
        }
        Move::Left => {
            for y in 0..BOARD_SIZE {
                for x in 0..BOARD_SIZE-1 {
                    if gs.board[x][y] == 0 {
                        continue;
                    }
                    if gs.board[x][y] == gs.board[x+1][y] {
                        gs.board[x+1][y] = 0;
                        gs.board[x][y] += 1;
                    }
                }
            }
        }
        Move::Right => {
            for y in 0..BOARD_SIZE {
                for x in (1..BOARD_SIZE).rev() {
                    if gs.board[x][y] == 0 {
                        continue;
                    }
                    if gs.board[x][y] == gs.board[x-1][y] {
                        gs.board[x-1][y] = 0;
                        gs.board[x][y] += 1;
                    }
                }
            }
        }
    }
    gs
}

// Return the state of the board after making the specified move.
// Seems to work.
pub fn after_move(mut gs: BoardState, m: &Move) -> BoardState {
    gs = after_slide_tiles(gs, m);
    gs = after_merge_tiles(gs, m);
    gs = after_slide_tiles(gs, m);
    gs
}