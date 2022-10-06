#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;
use std::char;
use rand::prelude::*;

pub const BOARD_SIZE: usize = 4;

// The state of the game.
// Contains a 4x4 grid of integers.
// The integer n represents the tile 2^n.
// The integer 0 represents absence of a tile.
// This is enough to hold all tiles obtainable in a normal 2048 game.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoardState {
    pub board: [[u8;BOARD_SIZE];BOARD_SIZE]
}

impl BoardState {
    pub fn new(b: [[u8;4];4]) -> Self {
        Self {
            board: b
        }
    }

    // Return the tile value at the specified location.
    pub fn get(&self, perp: usize, para: usize, m: &Move) -> u8 {
        match m {
            Move::Up => self.board[perp][para],
            Move::Down => self.board[perp][BOARD_SIZE-para-1],
            Move::Left => self.board[para][perp],
            Move::Right => self.board[BOARD_SIZE-para-1][perp]
        }
    }

    // Set the tile value at the specified location.
    pub fn set(&mut self, perp: usize, para: usize, m: &Move, val: u8) {
        match m {
            Move::Up => self.board[perp][para] = val,
            Move::Down => self.board[perp][BOARD_SIZE-para-1] = val,
            Move::Left => self.board[para][perp] = val,
            Move::Right => self.board[BOARD_SIZE-para-1][perp] = val
        }
    }

    pub fn compact_print(&self) {
        println!("+----+");
        for y in 0..BOARD_SIZE {
            let mut line = String::from("");
            for x in 0..BOARD_SIZE {
                let tile = self.board[x][y];

                if tile == 0 { 
                    line.push(' ');
                } else {
                    match char::from_digit(tile.into(), 36) {
                        Some(c) => {
                            line.push(c);
                        },
                        None => {}
                    }
                }
            }
            println!("|{}|", line);
        }
        println!("+----+");
    }
}

// The moves a player can make.
#[derive(Debug, EnumIter)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

// Return the state of the board after sliding the tiles in the specified direction.
// Tested.
pub fn slide_tiles(gs: &mut BoardState, m: &Move){
    for pe in 0..BOARD_SIZE {
        let mut target_pr = 0;
        for pr in 0..BOARD_SIZE {
            if gs.get(pe, pr, m) != 0 {
                if target_pr != pr {
                    // Slide this tile to the first empty spot
                    gs.set(pe, target_pr, m, gs.get(pe, pr, m));
                    gs.set(pe, pr, m, 0);
                }

                target_pr += 1;
            }
        }
    }
}

// Return the board state after merging tiles in the specified direction.
// Has been tested.
pub fn merge_tiles(gs: &mut BoardState, m: &Move) {
    for pe in 0..BOARD_SIZE {
        for pr in 0..BOARD_SIZE-1 {
            if gs.get(pe, pr, m) == 0 {
                continue;
            }
            if gs.get(pe, pr, m) == gs.get(pe, pr+1, m) {
                gs.set(pe, pr+1, m, 0);
                gs.set(pe, pr, m, gs.get(pe, pr, m) + 1);
            }
        }
    }
}

// Return the state of the board after making the specified move.
// Seems to work.
pub fn after_move(mut gs: BoardState, m: &Move) -> BoardState {
    slide_tiles(&mut gs, m);
    merge_tiles(&mut gs, m);
    slide_tiles(&mut gs, m);
    gs
}

// Return the board after spawning a random tile.
pub fn after_tile_spawn(mut gs: BoardState) -> BoardState {
    let mut rng = thread_rng();
    let tile = if rng.gen_range(0.0..1.0) < 0.9 {1} else {2};

    let mut num_open_tiles = 0;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if gs.board[x][y] == 0 {
                num_open_tiles += 1;
            }
        }
    }

    let chosen_position = rng.gen_range(0..num_open_tiles);
    let mut cur_index = 0;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if gs.board[x][y] != 0 {
                continue;
            }
            if cur_index == chosen_position {
                gs.board[x][y] = tile;
                return gs;
            }
            cur_index += 1;
        }
    }

    gs
}