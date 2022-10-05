use crate::prelude::*;

// Return the score, assuming that it is the player's turn.
// Uses the provided score function when it bottoms out.
pub fn player_side_score(gs: &BoardState, score: fn(&BoardState) -> f64, dead_score: f64, depth: u32) -> f64 {
    // We've bottomed out.
    if depth == 0 {
        return score(gs);
    }

    let mut scores = Vec::new();
    for m in Move::iter() {
        let am = after_move(*gs, &m);
        if am != *gs {
            scores.push(game_side_score(&am, score, depth-1));
        }
    }

    if scores.len() == 0 {
        dead_score
    } else {
        let mut max_score = scores[0];
        for score in scores {
            if score > max_score {
                max_score = score;
            }
        }
        max_score
    }

    

    // The highest value of all the possible moves.
    // Move::iter()
    //     .map(|m: Move| -> u32 {game_side_score(&after_move(*gs, &m), score, depth)})
    //     .max()
    //     .unwrap() // We can unwrap because we know there is at least one move

    // The highest value of all possible moves, ignoring moves that don't result in any changes.
    // Move::iter()
    //     .map(|m: Move| -> BoardState {after_move(*gs, &m)})
    //     .filter(|b: &BoardState| -> bool {*b != *gs})
    //     .map(|b: BoardState| -> f64 {game_side_score(&b, score, depth-1)})
    //     .max()
    //     .unwrap()

    // for dir in Move::iter() {
    //     println!("{:?}", dir);
    // }

    
}

// Return the score, assuming that it is the game's turn.
// Uses the provided score function when it bottoms out.
pub fn game_side_score(gs: &BoardState, score: fn(&BoardState) -> f64, dead_score: f64, depth: u32) -> f64 {
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if gs.board[x][y] != 0 {
                continue;
            }

            let mut scores = Vec::new();
            for val in 1..=2 {
                let mut after_tile = *gs;
                after_tile.board[x][y] = val;
                let score = player_side_score(&after_tile, score, dead_score, depth);
                scores.push(score);
            }
        }
    }

    0.0
}