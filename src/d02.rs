use std::io::BufRead;

use crate::{get_day_input, AOCSolver, Day, Part};

pub struct Solver;

impl AOCSolver for Solver {
    type Solution = u64;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Weapon {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Weapon {
    fn as_weapon(x: &str) -> Self {
        match x {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("{x} is not a valid move"),
        }
    }
}

#[derive(Clone, Copy)]
enum End {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl End {
    fn as_end(x: &str) -> Self {
        match x {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("{x} is not a valid move"),
        }
    }
}

pub struct Game {
    weapon: Weapon,
    end: End,
}

impl Game {
    const fn score(self) -> u64 {
        self.weapon as u64 + self.end as u64
    }
}

fn parse_games_by_attack(load_sample: bool) -> Vec<Game> {
    let file = get_day_input(Day::new(2), load_sample);
    let reader = std::io::BufReader::new(file);

    let mut parsed_games = Vec::new();

    for line in reader.lines().flatten() {
        let xs = line
            .split_whitespace()
            .map(Weapon::as_weapon)
            .collect::<Vec<_>>();
        let end_game = match (xs[1], xs[0]) {
            (Weapon::Rock, Weapon::Paper)
            | (Weapon::Paper, Weapon::Scissors)
            | (Weapon::Scissors, Weapon::Rock) => End::Loss,
            (Weapon::Rock, Weapon::Rock)
            | (Weapon::Paper, Weapon::Paper)
            | (Weapon::Scissors, Weapon::Scissors) => End::Draw,
            (Weapon::Rock, Weapon::Scissors)
            | (Weapon::Paper, Weapon::Rock)
            | (Weapon::Scissors, Weapon::Paper) => End::Win,
        };

        let game = Game {
            weapon: xs[1],
            end: end_game,
        };

        parsed_games.push(game);
    }

    parsed_games
}

fn parse_games_by_end_state(load_sample: bool) -> Vec<Game> {
    let file = get_day_input(Day::new(2), load_sample);
    let reader = std::io::BufReader::new(file);

    let mut parsed_games = Vec::new();

    for line in reader.lines().flatten() {
        let xs = line.split_whitespace().collect::<Vec<_>>();

        let (weapon, end_game) = (Weapon::as_weapon(xs[0]), End::as_end(xs[1]));

        let end_weapon = match (weapon, end_game) {
            (Weapon::Scissors, End::Win)
            | (Weapon::Rock, End::Draw)
            | (Weapon::Paper, End::Loss) => Weapon::Rock,
            (Weapon::Rock, End::Win)
            | (Weapon::Paper, End::Draw)
            | (Weapon::Scissors, End::Loss) => Weapon::Paper,
            (Weapon::Paper, End::Win)
            | (Weapon::Scissors, End::Draw)
            | (Weapon::Rock, End::Loss) => Weapon::Scissors,
        };

        let game = Game {
            end: end_game,
            weapon: end_weapon,
        };

        parsed_games.push(game);
    }

    parsed_games
}

fn solve_part_one() -> u64 {
    let games = parse_games_by_attack(false);

    games.into_iter().map(Game::score).sum()
}

fn solve_part_two() -> u64 {
    let games = parse_games_by_end_state(false);

    games.into_iter().map(Game::score).sum()
}

#[cfg(test)]
mod tests {
    use crate::d02::{parse_games_by_attack, parse_games_by_end_state, Game};

    #[test]
    fn solve_sample_one() {
        let games = parse_games_by_attack(true);

        let val = games.into_iter().map(Game::score).sum::<u64>();

        assert_eq!(15u64, val);
    }

    #[test]
    fn solve_sample_two() {
        let games = parse_games_by_end_state(true);

        let val = games.into_iter().map(Game::score).sum::<u64>();

        assert_eq!(12u64, val);
    }
}
