use crate::prelude::*;

pub fn play_2048(mut board: BoardState, config: &ScoreConfig, depth: u32, board_handler: fn(&BoardState) -> ()) {
    loop {
        board_handler(&board);

        // Choose the best move, if there is any
        let mut chosen_move: Option<Move> = None;
        let mut highest_score = 0.0;
        for m in Move::iter() {
            let am = after_move(board, &m);
            if am != board {
                let score = game_side_score(&am, config, depth);
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