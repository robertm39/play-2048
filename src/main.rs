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
    loop {
        let mut board = BoardState::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
        board = after_tile_spawn(board);
        board = after_tile_spawn(board);

        let min_weight = 0.6;

        let corner_weight = 0.5;
        let center_weight = 1.3125;

        let emp_non_adj = -0.5; //-0.5
        let sur_non_adj = -0.5625; //-0.5625

        let sur_adj = 0.0;
        let emp_adj = 0.625;

        let dead_score = -100.0;

        let score = |b: &mut BoardState| -> f64 {
            num_tiles(b) +
            corner_weight * highest_tile_in_corner_score(b) +
            center_weight * highest_tile_in_center_score(b) +
            adjacent_tiles_score(b, emp_non_adj, emp_adj, sur_non_adj, sur_adj)
        };

        let comb = |b: &Vec<WeightedScore>| -> f64 {
            min_weight * min_score(b) + (1.0-min_weight) * mean_score(b)
        };

        let config = ScoreConfig::new(score, dead_score, max_score, comb);

        //println!(); b.compact_print();
        // let end_board = play_2048(board, &config, 2, |_: &BoardState| {});
        let end_board = play_2048(board, &config, 2, |b: &BoardState| {println!(); b.compact_print();});

        let total_score = f64::from(end_board.total_tiles());
        println!("Total: {}, Highest: {}", total_score, end_board.get_highest_tile());
    }
}

// Good weights:
// 
// min_weight: 0.6, I guess, but it doesn't seem to make a big difference
// 
// num_tiles is fixed at 1.0
// corner_weight: 0.5 (was 1.0625)
// center_weight: 1.3125 (was 0.5)
// 
// emp_non_adj: -0.5
// sur_non_adj: -0.5625
// sur_adj: 0.0, I guess
// emp_adj: 0.625

const TRIALS_PER_VALUE: u32 = 1024*2;

fn trials_run() {
    let min_weight = 0.6;

    let corner_weight = 0.5;
    let center_weight = 1.3125;

    let emp_non_adj = -0.5; //-0.5
    let mut sur_non_adj = -1.0; //-0.5625

    let sur_adj = 0.0;
    let emp_adj = 0.625;

    let dead_score = -100.0;

    for _ in 0..33 {
        let mut total_score = 0.0;

        for _ in 0..TRIALS_PER_VALUE {
            let mut board = BoardState::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
            
            board = after_tile_spawn(board);
            board = after_tile_spawn(board);

            let score = |b: &mut BoardState| -> f64 {
                num_tiles(b) +
                corner_weight * highest_tile_in_corner_score(b) +
                center_weight * highest_tile_in_center_score(b) +
                adjacent_tiles_score(b, emp_non_adj, emp_adj, sur_non_adj, sur_adj)
            };

            let comb = |b: &Vec<WeightedScore>| -> f64 {
                min_weight * min_score(b) + (1.0-min_weight) * mean_score(b)
            };

            let config = ScoreConfig::new(score, dead_score, max_score, comb);

            let end_board = play_2048(board, &config, 2, |_: &BoardState| {});

            total_score += f64::from(end_board.total_tiles());
        }

        let av_score: f64 = total_score / f64::from(TRIALS_PER_VALUE);
        println!("{}\t{}", sur_non_adj, av_score);

        sur_non_adj += 1.0/32.0;

    }
}

fn main() {
    // trials_run();
    // timed_run();
    // test_run();
    plain_run();
}
