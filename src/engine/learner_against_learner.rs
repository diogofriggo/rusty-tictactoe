use std::collections::HashMap;

use super::flow;
use super::flow::Mechanics;
use super::{Game, Play, Player};

pub struct LearnerAgainstLearner {
    pub you_state_value: HashMap<Vec<Play>, f64>,
    pub opponent_state_value: HashMap<Vec<Play>, f64>,
}

impl Mechanics for LearnerAgainstLearner {
    fn get_position(&self, game: &mut Game) -> usize {
        match game.current_player {
            Player::You => {
                flow::get_position_from_learner_against_non_human(game, &self.you_state_value)
            }
            Player::Opponent => {
                flow::get_position_from_learner_against_non_human(game, &self.opponent_state_value)
            }
        }
    }

    fn update_state_value(&mut self, game: Game) {
        flow::update_state_value(&Player::You, &mut self.you_state_value, game.clone());
        flow::update_state_value(&Player::Opponent, &mut self.opponent_state_value, game);
    }

    fn should_report_to_stdout(&self) -> bool {
        false
    }
}
