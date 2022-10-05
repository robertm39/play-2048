#![warn(clippy::all, clippy::pedantic)]

mod game;
mod minmax;

mod prelude {
    pub use crate::game::*;
    pub use crate::minmax::*;
    pub use strum::IntoEnumIterator;
    pub use strum_macros::EnumIter;
}

use prelude::*;

fn main() {
    let board = BoardState::new([[1, 0, 2, 0], [0, 0, 2, 3], [1, 0, 2, 0], [0, 0, 2, 3]]);
    println!("{:?}", board);
    player_side_score(&board, |_: &BoardState| -> f64 {0.0}, 0.0, 1);
    // println!("{:?}", after_move(board, &Move::Right));
}
