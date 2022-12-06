use std::io::BufRead;

use crate::{get_day_input, Day, Part, Solver};

pub struct D2Solver;

impl Solver for D2Solver {
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
            _ => panic!("{} is not a valid move", x),
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
            _ => panic!("{} is not a valid move", x),
        }
    }
}

pub struct Game {
    weapon: Weapon,
    end: End,
}

impl Game {
    fn score(self) -> u64 {
        self.weapon as u64 + self.end as u64
    }
}

fn parse_games(load_sample: bool) -> Vec<Game> {
    let file = get_day_input(Day::new(2), load_sample);
    let reader = std::io::BufReader::new(file);

    let mut parsed_games = Vec::new();

    for line in reader.lines().flatten() {
        let xs = line
            .split_whitespace()
            .map(Weapon::as_weapon)
            .collect::<Vec<_>>();
        let end_game = match (xs[0], xs[1]) {
            (Weapon::Rock, Weapon::Rock) => End::Draw,
            (Weapon::Paper, Weapon::Paper) => End::Draw,
            (Weapon::Scissors, Weapon::Scissors) => End::Draw,
            (Weapon::Rock, Weapon::Paper) => End::Win,
            (Weapon::Paper, Weapon::Scissors) => End::Win,
            (Weapon::Scissors, Weapon::Rock) => End::Win,
            (Weapon::Rock, Weapon::Scissors) => End::Loss,
            (Weapon::Paper, Weapon::Rock) => End::Loss,
            (Weapon::Scissors, Weapon::Paper) => End::Loss,
        };

        let game = Game {
            weapon: xs[1],
            end: end_game,
        };

        parsed_games.push(game);
    }

    parsed_games
}

fn parse_games_different(load_sample: bool) -> Vec<Game> {
    let file = get_day_input(Day::new(2), load_sample);
    let reader = std::io::BufReader::new(file);

    let mut parsed_games = Vec::new();

    for line in reader.lines().flatten() {
        let xs = line.split_whitespace().collect::<Vec<_>>();

        let (weapon, end_game) = (Weapon::as_weapon(xs[0]), End::as_end(xs[1]));

        let end_weapon = match (weapon, end_game) {
            (Weapon::Rock, End::Win) => Weapon::Paper,
            (Weapon::Paper, End::Win) => Weapon::Scissors,
            (Weapon::Scissors, End::Win) => Weapon::Rock,
            (Weapon::Rock, End::Draw) => Weapon::Rock,
            (Weapon::Paper, End::Draw) => Weapon::Paper,
            (Weapon::Scissors, End::Draw) => Weapon::Scissors,
            (Weapon::Rock, End::Loss) => Weapon::Scissors,
            (Weapon::Paper, End::Loss) => Weapon::Rock,
            (Weapon::Scissors, End::Loss) => Weapon::Paper,
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
    let games = parse_games(false);

    games.into_iter().map(|g| g.score()).sum()
}

fn solve_part_two() -> u64 {
    let games = parse_games_different(false);

    games.into_iter().map(|g| g.score()).sum()
}

#[test]
fn solve_sample_one() {
    let games = parse_games(true);

    let val = games.into_iter().map(|g| g.score()).sum();

    assert_eq!(15u64, val);
}

#[test]
fn solve_sample_two() {
    let games = parse_games_different(true);

    let val = games.into_iter().map(|g| g.score()).sum();

    assert_eq!(12u64, val);
}
