use rand::Rng;
use std::collections::HashMap;

use super::{Game, Play, Player};

pub static EXPLORATION_RATE: f64 = 0.3;
pub static LEARNING_RATE: f64 = 0.4;
pub static DECAY: f64 = 0.9;

pub trait Mechanics {
    fn get_position(&self, game: &mut Game) -> usize;
    fn update_state_value(&mut self, game: Game);
    fn should_report_to_stdout(&self) -> bool;
}

pub fn get_position_from_learner_against_human(
    game: &mut Game,
    state_value: &HashMap<Vec<Play>, f64>,
) -> usize {
    get_position_from_greedy_learner(game, state_value)
}

pub fn get_position_from_learner_against_non_human(
    game: &mut Game,
    state_value: &HashMap<Vec<Play>, f64>,
) -> usize {
    let mut rng = rand::thread_rng();
    let random: f64 = rng.gen(); //0.0..1.0 not 0.0..=1.0

    match random <= EXPLORATION_RATE {
        true => game.get_random_position(), // exploratory
        false => get_position_from_greedy_learner(game, state_value), //greedy
    }
}

pub fn get_position_from_greedy_learner(
    game: &mut Game,
    state_value: &HashMap<Vec<Play>, f64>,
) -> usize {
    let maybe_pair = game
        .get_available_positions()
        .iter()
        .map(|&position| {
            game.plays.push(Play {
                position,
                player: game.current_player,
            });
            let maybe_value = state_value.get(&game.plays);
            game.plays.pop();
            (position, maybe_value)
        })
        .filter_map(|(position, maybe_value)| maybe_value.map(|value| (position, value)))
        .max_by(|(_, value_a), (_, value_b)| value_a.partial_cmp(value_b).unwrap());

    match maybe_pair {
        Some((position, _value)) => position,
        None => game.get_random_position(),
    }
}

pub fn get_position_from_user(game: &Game) -> usize {
    loop {
        if let Some(position) = read_position_from_user() {
            if !super::POSITIONS.contains(&position) {
                println!("This move does not exist! Choose one between 1-9");
                continue;
            }

            if game.is_position_occupied(position) {
                println!("This move has already been made! Choose an available one.");
                continue;
            }

            return position;
        }
        println!("Invalid input, try again!");
    }
}

pub fn read_position_from_user() -> Option<usize> {
    let mut line = String::new();
    let result = std::io::stdin().read_line(&mut line);
    match result {
        Ok(_) => match line.trim().parse::<usize>() {
            Ok(number) => Some(number),
            _ => None,
        },
        _ => None,
    }
}

pub fn update_state_value(
    current_player: &Player,
    state_value: &mut HashMap<Vec<Play>, f64>,
    mut game: Game,
) {
    let mut reward = compute_reward(current_player, &game);
    loop {
        let mut value = match state_value.get(&game.plays) {
            Some(&value) => value,
            None => 0.0,
        };
        value += LEARNING_RATE * (DECAY * reward - value);
        let _ = state_value.insert(game.plays.clone(), value);
        reward = value;

        match game.plays.pop() {
            Some(_) => {}
            None => break,
        }
    }
}

pub fn compute_reward(player: &Player, game: &Game) -> f64 {
    match game.get_winner() {
        Some(winner) => match (winner, player) {
            (Player::You, Player::You) => 1.0,
            (Player::Opponent, Player::Opponent) => 1.0,
            (Player::You, Player::Opponent) => 0.0,
            (Player::Opponent, Player::You) => 0.0,
        },
        None => match player {
            Player::You => 0.5,
            Player::Opponent => 0.1,
        },
    }
}
