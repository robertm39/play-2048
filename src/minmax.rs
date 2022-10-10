use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WeightedScore {
    pub score: f64,
    pub weight: f64,
}

impl WeightedScore {
    pub fn new(score: f64, weight: f64) -> Self{
        Self {
            score,
            weight,
        }
    }
}

// Info for determining the score of a board.
pub struct ScoreConfig<S, P, G> 
where
    S: Fn(&mut BoardState) -> f64,
    P: Fn(&Vec<f64>) -> f64,
    G: Fn(&Vec<WeightedScore>) -> f64,
{
    pub score_func: S,//fn(&BoardState) -> f64,
    pub dead_score: f64,
    pub player_agg: P,//fn(&Vec<f64>) -> f64,
    pub game_agg: G,//fn(&Vec<WeightedScore>) -> f64,
}

impl<S, P, G> ScoreConfig<S, P, G>
where
    S: Fn(&mut BoardState) -> f64,
    P: Fn(&Vec<f64>) -> f64,
    G: Fn(&Vec<WeightedScore>) -> f64,
{
    pub fn new(
        score_func: S,
        dead_score: f64,
        player_agg: P,
        game_agg: G,) -> Self {
        
        Self {
            score_func,
            dead_score,
            player_agg,
            game_agg,
        }
    }
}

// Return the score, assuming that it is the game's turn.
// Uses the provided score function when it bottoms out.
pub fn game_side_score<S, P, G>(gs: &mut BoardState, score_config: &ScoreConfig<S, P, G>, depth: u32) -> f64
where
    S: Fn(&mut BoardState) -> f64,
    P: Fn(&Vec<f64>) -> f64,
    G: Fn(&Vec<WeightedScore>) -> f64,
{
    let mut scores = Vec::new();
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if gs.board[x][y] != 0 {
                continue;
            }

            for val in 1..=2 {
                let mut after_tile = *gs;
                after_tile.board[x][y] = val;
                let score = player_side_score(&mut after_tile, score_config, depth);
                scores.push(WeightedScore::new(score, if val==1 {0.9} else {0.1}));
            }
        }
    }

    if scores.is_empty() {
        score_config.dead_score
    } else {
        (score_config.game_agg)(&scores)
    }
}

// Return the score, assuming that it is the player's turn.
// Uses the provided score function when it bottoms out.
pub fn player_side_score<S, P, G>(gs: &mut BoardState, score_config: &ScoreConfig<S, P, G>, depth: u32) -> f64
where
    S: Fn(&mut BoardState) -> f64,
    P: Fn(&Vec<f64>) -> f64,
    G: Fn(&Vec<WeightedScore>) -> f64,
{
    // We've bottomed out.
    if depth == 0 {
        return (score_config.score_func)(gs);
    }

    let mut scores = Vec::new();
    for m in Move::iter() {
        let mut am = after_move(*gs, &m);
        if am != *gs {
            scores.push(game_side_score(&mut am, score_config, depth-1));
        }
    }

    if scores.is_empty() {
        score_config.dead_score
    } else {
        (score_config.player_agg)(&scores)
        // let mut max_score = scores[0];
        // for score in scores {
        //     if score > max_score {
        //         max_score = score;
        //     }
        // }
        // max_score
    }
}

// each new tile supposedly has a 9 in 10 chance if being a 2
// this is also roughly what I observe



// Minmax functions that have a maximum width rather than a number of iterations
pub fn mw_game_side_score<S, P, G>(gs: &mut BoardState, score_config: &ScoreConfig<S, P, G>, width: u32) -> Option<f64>
where
    S: Fn(&mut BoardState) -> f64,
    P: Fn(&Vec<f64>) -> f64,
    G: Fn(&Vec<WeightedScore>) -> f64,
{
    // See how many possibilities there are
    let mut num_poss = 0;
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if gs.board[x][y] == 0 {
                num_poss += 2;
            }
        }
    }

    // If we don't have enough possibilities, fail
    if width < num_poss {
        return None;
    }

    // This shouldn't happen generally
    if num_poss == 0 {
        return Some(score_config.dead_score);
    }

    let poss_per = width / num_poss;
    let mut extra = width % num_poss;

    let mut scores = Vec::new();
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if gs.board[x][y] == 0 {
                for val in [1, 2] {
                    // See how many possibilities to allocate to the lower call
                    let mut poss = poss_per;
                    if extra > 0 {
                        poss += 1;
                        extra -= 1;
                    }

                    let mut after_tile = *gs;
                    after_tile.board[x][y] = val;
                    let score = mw_player_wide_score(gs, score_config, poss);
                    scores.push(WeightedScore::new(score, if val == 1 {0.9} else {0.1}));
                }
            }
        }
    }

    Some((score_config.game_agg)(&scores))
}

pub fn mw_player_wide_score<S, P, G>(gs: &mut BoardState, score_config: &ScoreConfig<S, P, G>, width: u32) -> f64
where
    S: Fn(&mut BoardState) -> f64,
    P: Fn(&Vec<f64>) -> f64,
    G: Fn(&Vec<WeightedScore>) -> f64,
{
    // Seeing how many possible moves there are
    let mut num_moves = 0;
    let mut poss_moves = Vec::new();
    for dir in Move::iter() {
        if after_move(*gs, &dir) != *gs {
            num_moves += 1;
            poss_moves.push(dir);
        }
    }

    // We don't have enough possibilities to explore all the moves,
    // so return the flat score of the board
    if width < num_moves {
        return (score_config.score_func)(gs);
    }

    // We don't have any moves, so return the dead score
    if num_moves == 0 {
        return score_config.dead_score;
    }

    let poss_per = width / num_moves;
    let mut extra = width % num_moves;

    let mut scores = Vec::new();
    for dir in poss_moves {
        let after = after_move(*gs, &dir);
        let mut num_poss = poss_per;
        if extra > 0 {
            num_poss += 1;
            extra -= 1;
        }

        // If we have enough possibilities to explore all responses, get that score
        // Otherwise, just use the flat score
        if let Some(score) = mw_game_side_score(gs, score_config, num_poss) {
            scores.push(score);
        } else {
            return (score_config.score_func)(gs);
        }
    }

    (score_config.player_agg)(&scores)
}