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

    let w_tile_sum = WeightedFunc::new(num_tiles, 1.0);
    let w_highest_corner = WeightedFunc::new(highest_tile_in_corner_score, 8.0);
    let w_highest_center = WeightedFunc::new(highest_tile_in_center_score, 0.0);

    let score = |b: &mut BoardState| -> f64 {
        w_tile_sum.get_score(b) +
        w_highest_corner.get_score(b) + 
        w_highest_center.get_score(b)
    };

    // println!("{}", score(&board));

    // let config = ScoreConfig::new(smarter_score, -100.0, max_score, comb_score);
    let config = ScoreConfig::new(score, -100.0, max_score, min_score);

    play_2048(board, &config, 2, |b: &BoardState| {b.compact_print()});
}
