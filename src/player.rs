use crate::prelude::*;
// use std::time::Instant;

// const MILLISECONDS_PER_MOVE: u128 = 20;

const ONE_LEVEL: u32 = 32*4;

const TARGET_WIDTH: u32 = ONE_LEVEL;

pub fn play_2048<S, P, G>(mut board: BoardState, config: &ScoreConfig<S, P, G>, _: u32, board_handler: fn(&BoardState) -> ()) -> BoardState
where
    S: Fn(&mut BoardState) -> f64,
    P: Fn(&Vec<f64>) -> f64,
    G: Fn(&Vec<WeightedScore>) -> f64,
{
    loop {
        // println!();
        board_handler(&board);
        // println!("{}", (config.score_func)(&mut board));
        // println!("{}", board.get_highest_tile());
        // println!();

        // Choose the best move, if there is any
        let mut chosen_move: Option<Move> = None;
        let mut highest_score = 0.0;
        for m in Move::iter() {
            let mut am = after_move(board, &m);
            if am != board {

                let mut score = 0.0;
                // Go for a certain width
                // let mut d = 0;
                for depth in 1..=100 {
                    // d = depth;
                    let score_and_width = rw_game_side_score(&mut am, config, depth);
                    score = score_and_width.score;
                    if score_and_width.width >= TARGET_WIDTH {
                        break;
                    }
                }
                // print!("{} ", d);

                // // Just go a constant number of levels deep
                // let score = game_side_score(&mut am, config, 2);
                
                // // Enough for at least one move and two moves after it
                // let score = mw_game_side_score(&mut am, config, 32 * 4 * 32 * 4 * 32).unwrap();

                // Keep computing scores at higher depth until enough time has passed
                // let now = Instant::now();
                // let mut score = 0.0;
                // let mut dp = 0;
                // for d in 2..=100 {
                // for d in [0] {
                    // dp = d;
                // score = game_side_score(&mut am, config, d);
                // if now.elapsed().as_millis() >= MILLISECONDS_PER_MOVE {
                //     break;
                // }
                // }
                // print!("{} ", dp);

                match chosen_move {
                    Some(_) => {
                        if score > highest_score {
                            chosen_move = Some(m);
                            highest_score = score;
                        }
                    },
                    None => {
                        chosen_move = Some(m);
                        highest_score = score;
                    }
                }
            }
        }

        match chosen_move {
            Some(d) => {
                board = after_move(board, &d);
                board = after_tile_spawn(board);
                // println!();
                // println!("{:?}", d);
            }
            None => {break;}
        }
    }
    // println!("The game is over.");
    board
}