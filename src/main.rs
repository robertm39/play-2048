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
    let mut board = BoardState::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    board = after_tile_spawn(board);
    board = after_tile_spawn(board);

    // let weighted_tile_sum = WeightedFunc::new(num_tiles, 1.0);

    let config = ScoreConfig::new(smarter_score, -100.0, max_score, comb_score);
    // let config = ScoreConfig::new(weighted_tile_sum.get_score, -100.0, max_score, comb_score);

    play_2048(board, &config, 2, |b: &BoardState| {b.compact_print()});
}
