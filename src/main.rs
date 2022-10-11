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
use std::time::Instant;


fn test_run() {
    let mut board = BoardState::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    board = after_tile_spawn(board);
    board = after_tile_spawn(board);

    let score = |b: &mut BoardState| -> f64 {
        num_tiles(b)
        // corner_weight * highest_tile_in_corner_score(b)
        // w_highest_corner.get_score(b)// + 
        //w_highest_center.get_score(b)
    };

    let config = ScoreConfig::new(score, 0.0, max_score, comb_score);

    for depth in 0..=4 {
        let score_width = rw_player_side_score(&mut board, &config, depth);
        println!("Depth: {}, score: {}, width: {}", depth, score_width.score, score_width.width);
    }
}

fn timed_run() {
    let mut board = BoardState::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    board = after_tile_spawn(board);
    board = after_tile_spawn(board);

    let score = |b: &mut BoardState| -> f64 {
        0.0 //num_tiles(b)
        // corner_weight * highest_tile_in_corner_score(b)
        // w_highest_corner.get_score(b)// + 
        //w_highest_center.get_score(b)
    };

    let config = ScoreConfig::new(score, 0.0, max_score, comb_score);

    let num_runs = 10000;

    let now = Instant::now();
    for _ in 0..num_runs {
        player_side_score(&mut board, &config, 2);
    }

    let elapsed = now.elapsed().as_millis();
    println!("{} milliseconds", elapsed);
}

fn plain_run() {
    let mut board = BoardState::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    board = after_tile_spawn(board);
    board = after_tile_spawn(board);

    let corner_weight = 1.0625;
    let center_weight = 0.5;
    let min_weight = 0.6;

    let emp_non_adj = -0.875;
    let sur_non_adj = -0.875;

    let sur_adj = 0.0;
    let emp_adj = 0.0;

    let dead_score = -100.0;

    let score = |b: &mut BoardState| -> f64 {
        num_tiles(b) +
        corner_weight * highest_tile_in_corner_score(b) +
        center_weight * highest_tile_in_center_score(b) +
        adjacent_tiles_score(b, emp_non_adj, emp_adj, sur_non_adj, sur_adj)
        // w_highest_corner.get_score(b)// + 
        //w_highest_center.get_score(b)
    };

    let comb = |b: &Vec<WeightedScore>| -> f64 {
        min_weight * min_score(b) + (1.0-min_weight) * mean_score(b)
    };

    let config = ScoreConfig::new(score, dead_score, max_score, comb);

    let end_board = play_2048(board, &config, 2, |b: &BoardState| {println!(); b.compact_print();});

    let total_score = f64::from(end_board.total_tiles());
    println!("{}", total_score);
}

// Good weights:
// 
// min_weight: 0.6, I guess, but it doesn't seem to make a big difference
// 
// num_tiles is fixed at 1.0
// corner_weight: 1.0625
// center_weight: 0.5
// 
// emp_non_adj: -0.5
// sur_non_adj: -0.5625
// sur_adj: 0.0, I guess
// emp_adj: 0.625

const TRIALS_PER_VALUE: u32 = 1024;

fn trials_run() {
    let min_weight = 0.6;

    let mut corner_weight = 0.0; //1.0625
    let center_weight = 0.5;

    let emp_non_adj = -0.5; 
    let sur_non_adj = -0.5625;

    let sur_adj = 0.0;
    let emp_adj = 0.625;

    let dead_score = -100.0;

    for _ in 0..33 {
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
                corner_weight * highest_tile_in_corner_score(b) +
                center_weight * highest_tile_in_center_score(b) +
                adjacent_tiles_score(b, emp_non_adj, emp_adj, sur_non_adj, sur_adj)
                // w_highest_corner.get_score(b)// + 
                //w_highest_center.get_score(b)
            };

            // println!("{}", score(&board));

            let comb = |b: &Vec<WeightedScore>| -> f64 {
                min_weight * min_score(b) + (1.0-min_weight) * mean_score(b)
            };

            // let config = ScoreConfig::new(smarter_score, -100.0, max_score, comb_score);
            let config = ScoreConfig::new(score, dead_score, max_score, comb);

            let end_board = play_2048(board, &config, 2, |_: &BoardState| {});
            // let base: u32 = 2;

            total_score += f64::from(end_board.total_tiles());
            // println!("{}\t{}\t{}", corner_weight, end_board.total_tiles(), base.pow(end_board.get_highest_tile().into()));
        }

        let av_score: f64 = total_score / f64::from(TRIALS_PER_VALUE);
        println!("{}\t{}", corner_weight, av_score);

        // corner_weight += 1.0/64.0;
        // min_weight += 1.0/64.0;
        // center_weight += 1.0/8.0;
        // same_adj_weight += 1.0/8.0;
        // dead_score -= 5.0;
        // sur_non_adj += 1.0/32.0;
        corner_weight += 1.0/8.0;

    }
}

fn main() {
    trials_run();
    // timed_run();
    // test_run();
    // plain_run();
}
