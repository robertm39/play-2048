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

const TRIALS_PER_VALUE: u32 = 1024*8;

fn plain_run() {
    let mut board = BoardState::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    board = after_tile_spawn(board);
    board = after_tile_spawn(board);
    
    let corner_weight = 10.0;

    let score = |b: &mut BoardState| -> f64 {
        num_tiles(b) +
        corner_weight * highest_tile_in_corner_score(b)
        // w_highest_corner.get_score(b)// + 
        //w_highest_center.get_score(b)
    };

    let config = ScoreConfig::new(score, -50.0, max_score, comb_score);

    let end_board = play_2048(board, &config, 2, |b: &BoardState| {b.compact_print(); println!();});

    let total_score = f64::from(end_board.total_tiles());
    println!("{}", total_score);
}

fn trials_run() {
    let mut corner_weight = -1.0;
    let min_weight = 0.5;
    for _ in 0..65 {
        let mut total_score = 0.0;

        for _ in 0..TRIALS_PER_VALUE {
            let mut board = BoardState::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
            
            board = after_tile_spawn(board);
            board = after_tile_spawn(board);

            //let w_tile_sum = WeightedFunc::new(num_tiles, 1.0);
            //let w_highest_corner = WeightedFunc::new(highest_tile_in_corner_score, corner_weight);
            //let w_highest_center = WeightedFunc::new(highest_tile_in_center_score, 0.0);

            let score = |b: &mut BoardState| -> f64 {
                num_tiles(b) +
                corner_weight * highest_tile_in_corner_score(b)
                // w_highest_corner.get_score(b)// + 
                //w_highest_center.get_score(b)
            };

            // println!("{}", score(&board));

            let comb = |b: &Vec<WeightedScore>| -> f64 {
                min_weight * min_score(b) + (1.0-min_weight) * mean_score(b)
            };

            // let config = ScoreConfig::new(smarter_score, -100.0, max_score, comb_score);
            let config = ScoreConfig::new(score, -100.0, max_score, comb);

            let end_board = play_2048(board, &config, 2, |_: &BoardState| {});
            // let base: u32 = 2;

            total_score += f64::from(end_board.total_tiles());
            // println!("{}\t{}\t{}", corner_weight, end_board.total_tiles(), base.pow(end_board.get_highest_tile().into()));
        }

        let av_score: f64 = total_score / f64::from(TRIALS_PER_VALUE);
        println!("{}\t{}", corner_weight, av_score);

        corner_weight += 1.0/16.0;
        // min_weight += 1.0/16.0;

    }
}

fn main() {
    trials_run();
}
