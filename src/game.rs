#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

const BOARD_SIZE: usize = 4;

// The state of the game.
// Contains a 4x4 grid of integers.
// The integer n represents the tile 2^n.
// The integer 0 represents absence of a tile.
// This is enough to hold all tiles obtainable in a normal 2048 game.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoardState {
    board: [[u8;BOARD_SIZE];BOARD_SIZE]
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