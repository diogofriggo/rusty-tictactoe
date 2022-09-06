use std::collections::HashMap;

use crate::engine::flow;

use super::{flow::Mechanics, Game, Play, Player};

pub struct HumanAgainstLearner {
    pub opponent_state_value: HashMap<Vec<Play>, f64>,
}

impl Mechanics for HumanAgainstLearner {
    fn get_position(&self, game: &mut Game) -> usize {
        if game.current_player == Player::You {
            println!("It's your turn, choose a cell by typing its number");
            return flow::get_position_from_user(game);
        }
        flow::get_position_from_learner_against_human(game, &self.opponent_state_value)
    }

    fn update_state_value(&mut self, _game: Game) {}

    fn should_report_to_stdout(&self) -> bool {
        true
    }
}
