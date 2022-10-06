use crate::prelude::*;
use std::time::Instant;

const MILLISECONDS_PER_MOVE: u128 = 100;

pub fn play_2048(mut board: BoardState, config: &ScoreConfig, _: u32, board_handler: fn(&BoardState) -> ()) {
    loop {
        board_handler(&board);

        // Choose the best move, if there is any
        let mut chosen_move: Option<Move> = None;
        let mut highest_score = 0.0;
        for m in Move::iter() {
            let am = after_move(board, &m);
            if am != board {

                // Keep computing scores at higher depth until enough time has passed
                let now = Instant::now();
                let mut score = 0.0;
                let mut dp = 0;
                for d in 0..=100 {
                    dp = d;
                    score = game_side_score(&am, config, d);
                    if now.elapsed().as_millis() >= MILLISECONDS_PER_MOVE {
                        break;
                    }
                }
                println!("{}", dp);

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
            }
            None => {break;}
        }
    }
    println!("The game is over.");
}