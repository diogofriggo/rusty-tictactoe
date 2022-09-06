use std::collections::HashMap;

use super::flow;
use super::flow::Mechanics;
use super::{Game, Player, Play};

pub struct LearnerAgainstRandom {
    pub you_state_value: HashMap<Vec<Play>, f64>,
}

impl Mechanics for LearnerAgainstRandom {
    fn get_position(&self, game: &mut Game) -> usize {
        match game.current_player {
            Player::You => flow::get_position_from_learner_against_non_human(game, &self.you_state_value),
            Player::Opponent => game.get_random_position(),
        }
    }

    fn update_state_value(&mut self, game: Game) {
        flow::update_state_value(&Player::You, &mut self.you_state_value, game);
    }

    fn should_report_to_stdout(&self) -> bool {
        false
    }
}