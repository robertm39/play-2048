#![warn(clippy::all, clippy::pedantic)]

mod game;
mod minmax;
mod scores;
mod aggs;
mod player;

mod prelude {
    pub use crate::game::*;
    pub use crate::minmax::*;
    pub use crate::scores::*;
    pub use crate::aggs::*;
    pub use crate::player::*;

    // I'll want to get rid of these
    pub use strum::IntoEnumIterator;
    pub use strum_macros::EnumIter;
}

use prelude::*;

fn main() {
    let board = BoardState::new([[1, 0, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);

    // for _ in 0..10 {
    //     let b = after_tile_spawn(board);
    //     b.compact_print();
    // }

    let config = ScoreConfig::new(smarter_score, -100.0, max_score, mean_score);

    play_2048(board, &config, 2, |b: &BoardState| {b.compact_print()});

    // for depth in 0..=3 {
    //     // let score = player_side_score(&board, |_: &BoardState| -> f64 {0.0}, 0.0, depth);
    //     let score = player_side_score(&board, &config, depth);
    //     println!("{}: {}", depth, score);
    // }
    // println!("{:?}", after_move(board, &Move::Right));
}
