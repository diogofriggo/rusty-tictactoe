use std::collections::HashMap;
use std::time::Instant;

mod engine;

fn main() {
    // # How to always win or draw:
    // If you start: choose a corner
    // If the opponent chooses the center the game will either tie or you'll win
    // If the opponent chooses anything else, he'll lose
    // choose the nearest other corner to your first move
    // they'll be forced to play in the middle
    // you then go for the other corner
    // If your second move is instead on the diagonal, you'll end up in a draw

    // Uncomment below to play against a random-play opponent

    // let mut mechanics = engine::learner_against_random::LearnerAgainstRandom {
    //     you_state_value: HashMap::new(),
    // };

    // Let the machine play against another machine player to learn tic-tac-toe!

    let mut mechanics = engine::learner_against_learner::LearnerAgainstLearner {
        you_state_value: HashMap::with_capacity(160000),
        opponent_state_value: HashMap::with_capacity(160000),
    };

    let start = Instant::now();
    for _ in 0..1_000_000 {
        engine::run(&mut mechanics);
    }

    println!(
        "{:?}s {} {}",
        start.elapsed(),
        mechanics.you_state_value.len(),
        mechanics.opponent_state_value.len()
    );

    // Now see if you can beat the machine!

    let mut mechanics = engine::human_against_learner::HumanAgainstLearner {
        opponent_state_value: mechanics.opponent_state_value,
    };

    for _ in 0..100 {
        engine::run(&mut mechanics);
    }
}
