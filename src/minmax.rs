use crate::prelude::*;

// Return the score, assuming that it is the player's turn.
// Uses the provided score function when it bottoms out.
pub fn player_side_score(gs: &BoardState, score: fn(&BoardState) -> u32, depth: u32) -> u32 {
    // We've bottomed out.
    if depth == 0 {
        return score(gs);
    }

    // The highest value of all the possible moves.
    // Move::iter()
    //     .map(|m: Move| -> u32 {game_side_score(&after_move(*gs, &m), score, depth)})
    //     .max()
    //     .unwrap() // We can unwrap because we know there is at least one move

    // The highest value of all possible moves, ignoring moves that don't result in any changes.
    Move::iter()
        .map(|m: Move| -> BoardState {after_move(*gs, &m)})
        .filter(|b: &BoardState| -> bool {*b != *gs})
        .map(|b: BoardState| -> u32 {game_side_score(&b, score, depth-1)})
        .max()
        .unwrap()

    // for dir in Move::iter() {
    //     println!("{:?}", dir);
    // }

    
}

// Return the score, assuming that it is the game's turn.
// Uses the provided score function when it bottoms out.
pub fn game_side_score(gs: &BoardState, score: fn(&BoardState) -> u32, depth: u32) -> u32 {
    0 // TODO
}