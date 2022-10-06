use crate::prelude::*;

// Player aggs.
// Computes the maximum of the given scores.
pub fn max_score(scores: &Vec<f64>) -> f64{
    let mut max_s = scores[0];
    for &score in scores {
        if score > max_s {
            max_s = score;
        }
    }

    max_s
}

// Game aggs.
// Computes the weighted mean of the given weighted scores.
pub fn mean_score(scores: &Vec<WeightedScore>) -> f64 {
    let mut total_weight = 0.0;
    let mut weighted_sum = 0.0;

    for &score in scores {
        total_weight += score.weight;
        weighted_sum += score.weight * score.score;
    }

    weighted_sum / total_weight
}

// Computes the minimum of the given scores.
pub fn min_score(scores: &Vec<WeightedScore>) -> f64{
    let mut min_s = scores[0].score;
    for &score in scores {
        if score.score < min_s {
            min_s = score.score;
        }
    }

    min_s
}

// Returns a combination of the minimum and the mean scores.
pub fn comb_score(scores: &Vec<WeightedScore>) -> f64 {
    return (mean_score(scores) + min_score(scores)) / 2.0;
}