use std::fmt::Display;

use colored::Colorize;
use rand::Rng;

pub mod flow;
pub mod human_against_learner;
pub mod human_against_random;
pub mod learner_against_learner;
pub mod learner_against_random;

use flow::Mechanics;

static POSITIONS: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

static WINNING_SEQUENCES: &[[usize; 3]; 8] = &[
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
    [1, 5, 9],
    [3, 5, 7],
    [1, 4, 7],
    [2, 5, 8],
    [3, 6, 9],
];

pub fn run(mechanics: &mut impl Mechanics) {
    let mut game = Game {
        plays: Vec::with_capacity(9),
        current_player: Player::You,
    };

    while !game.is_over() {
        if mechanics.should_report_to_stdout() {
            println!("Next round!");
            println!("{}", game);
        }
        let position = mechanics.get_position(&mut game);
        game.plays.push(Play {
            position,
            player: game.current_player,
        });
        game.current_player = match game.current_player {
            Player::You => Player::Opponent,
            Player::Opponent => Player::You,
        };
    }
    if mechanics.should_report_to_stdout() {
        println!("{}", game);
        if game.is_draw() {
            println!("Draw! No one won the game");
        } else {
            println!("{:?} won the game!", game.get_winner().unwrap());
        }
    }
    mechanics.update_state_value(game);
}

#[derive(Clone)]
pub struct Game {
    pub plays: Vec<Play>,
    pub current_player: Player,
}

impl Game {
    pub fn is_over(&self) -> bool {
        self.is_draw() || self.get_winner().is_some()
    }

    pub fn is_draw(&self) -> bool {
        self.plays.len() == 9 && self.get_winner().is_none()
    }

    pub fn get_winner(&self) -> Option<Player> {
        for winning_sequence in WINNING_SEQUENCES {
            let players: Vec<Option<Player>> = winning_sequence
                .iter()
                .map(|&i| self.get_player_at_position(i))
                .collect();

            if players.iter().all(|player| player == &Some(Player::You)) {
                return Some(Player::You);
            }
            if players
                .iter()
                .all(|player| player == &Some(Player::Opponent))
            {
                return Some(Player::Opponent);
            }
        }
        None
    }

    pub fn get_available_positions(&self) -> Vec<usize> {
        POSITIONS
            .into_iter()
            .filter(|&position| !self.is_position_occupied(position))
            .collect()
    }

    pub fn is_position_occupied(&self, position: usize) -> bool {
        self.plays.iter().any(|play| play.position == position)
    }

    pub fn get_player_at_position(&self, position: usize) -> Option<Player> {
        for play in &self.plays {
            if play.position == position {
                return Some(play.player);
            }
        }
        None
    }

    pub fn get_random_position(&self) -> usize {
        let available_positions = self.get_available_positions();
        let n: usize = available_positions.len();
        let mut rng = rand::thread_rng();
        let random_choice: usize = rng.gen_range(0..n);
        available_positions[random_choice]
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Play {
    pub position: usize,
    pub player: Player,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Player {
    You,
    Opponent,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p1 = get_symbol(1, &self.plays);
        let p2 = get_symbol(2, &self.plays);
        let p3 = get_symbol(3, &self.plays);
        let p4 = get_symbol(4, &self.plays);
        let p5 = get_symbol(5, &self.plays);
        let p6 = get_symbol(6, &self.plays);
        let p7 = get_symbol(7, &self.plays);
        let p8 = get_symbol(8, &self.plays);
        let p9 = get_symbol(9, &self.plays);

        write!(f, " {} | {} | {}\n", p1, p2, p3)?;
        write!(f, "-----------\n")?;
        write!(f, " {} | {} | {}\n", p4, p5, p6)?;
        write!(f, "-----------\n")?;
        write!(f, " {} | {} | {}\n", p7, p8, p9)?;
        write!(f, "-----------")
    }
}

pub fn get_symbol(position: usize, plays: &Vec<Play>) -> String {
    for play in plays {
        if position == play.position {
            return match play.player {
                Player::You => "X".red().to_string(),
                Player::Opponent => "O".blue().to_string(),
            };
        }
    }
    position.to_string()
}
