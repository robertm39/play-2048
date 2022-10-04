#![warn(clippy::all, clippy::pedantic)]

mod game;

mod prelude {
    pub use crate::game::*;
}

use prelude::*;

fn main() {
    let board = BoardState::new([[1, 0, 3, 0], [1, 0, 3, 0], [0, 0, 3, 0], [0, 0, 3, 0]]);
    println!("{:?}", board);
    println!("{:?}", after_move(board, &Move::Left));
}
