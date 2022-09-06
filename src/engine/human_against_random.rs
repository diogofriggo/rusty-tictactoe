use crate::engine::flow::get_position_from_user;

use super::{flow::Mechanics, Game, Player};

pub struct HumanAgainstRandom;

impl Mechanics for HumanAgainstRandom {
    fn get_position(&self, game: &mut Game) -> usize {
        if game.current_player == Player::You {
            println!("It's your turn, choose a cell by typing its number");
            return get_position_from_user(game);
        }
        game.get_random_position()
    }

    fn update_state_value(&mut self, _game: Game) {}

    fn should_report_to_stdout(&self) -> bool {
        true
    }
}
