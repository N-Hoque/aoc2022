use std::{collections::HashSet, io::BufRead};

use crate::{get_day_input, Day, Part, Solver};

pub struct D9Solver;

impl Solver for D9Solver {
    type Solution = usize;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Move {
    direction: Direction,
    steps: u64,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn clamp(self) -> Self {
        Self {
            x: self.x.clamp(-1, 1),
            y: self.y.clamp(-1, 1),
        }
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    pub fn move_in_direction(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    pub fn update(&mut self, direction: Direction) {
        self.knots[0] = self.knots[0].move_in_direction(direction);

        for knot_idx in 1..self.knots.len() {
            let tail_diff = self.knots[knot_idx - 1] - self.knots[knot_idx];

            if tail_diff.y == 0 {
                if tail_diff.x == 2 {
                    self.knots[knot_idx].x += 1;
                } else if tail_diff.x == -2 {
                    self.knots[knot_idx].x -= 1;
                }
            } else if tail_diff.x == 0 {
                if tail_diff.y == 2 {
                    self.knots[knot_idx].y += 1;
                } else if tail_diff.y == -2 {
                    self.knots[knot_idx].y -= 1;
                }
            } else if !self.are_points_touching(knot_idx) {
                self.knots[knot_idx] = self.knots[knot_idx] + tail_diff.clamp();
            }
        }
    }

    fn are_points_touching(&self, knot_idx: usize) -> bool {
        let range = -1..=1;

        let diff = self.knots[knot_idx] - self.knots[knot_idx - 1];

        range.contains(&diff.x) && range.contains(&diff.y)
    }
}

pub struct State {
    rope: Rope,
    visited_positions: HashSet<Position>,
}

impl State {
    pub fn new(knots: usize) -> Self {
        assert!(knots >= 2, "There must be at least two knots in the rope");

        Self {
            rope: Rope {
                knots: vec![Position::default(); knots],
            },
            visited_positions: HashSet::default(),
        }
    }

    pub fn update(&mut self, r#move: Move) {
        for _ in 0..r#move.steps {
            self.rope.update(r#move.direction);
            self.visited_positions
                .insert(*self.rope.knots.last().unwrap());
        }
    }

    pub fn num_visited_positions(&self) -> usize {
        self.visited_positions.len()
    }
}

fn parse_movements(load_sample: bool) -> Vec<Move> {
    let file = get_day_input(Day::new(9), load_sample);

    let reader = std::io::BufReader::new(file);

    let mut moves = Vec::new();

    for line in reader.lines().flatten() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        let motion = Move {
            direction: match parts[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            },
            steps: parts[1].parse().unwrap(),
        };
        moves.push(motion);
    }

    moves
}

fn solve_part_one() -> usize {
    let instructions = parse_movements(false);

    let mut state = State::new(2);

    for r#move in &instructions {
        state.update(*r#move);
    }

    state.num_visited_positions()
}

fn solve_part_two() -> usize {
    let instructions = parse_movements(false);

    let mut state = State::new(10);

    for r#move in &instructions {
        state.update(*r#move);
    }

    state.num_visited_positions()
}

#[test]
fn solve_sample_one() {
    let instructions = parse_movements(true);

    let mut state = State::new(2);

    for r#move in &instructions {
        state.update(*r#move);
    }

    assert_eq!(state.num_visited_positions(), 13);
}

#[test]
fn solve_sample_two() {
    let instructions = parse_movements(true);

    let mut state = State::new(10);

    for r#move in &instructions {
        state.update(*r#move);
    }

    assert_eq!(state.num_visited_positions(), 1);
}

#[test]
fn solve_sample_three() {
    let instructions = vec![
        Move {
            direction: Direction::Right,
            steps: 5,
        },
        Move {
            direction: Direction::Up,
            steps: 8,
        },
        Move {
            direction: Direction::Left,
            steps: 8,
        },
        Move {
            direction: Direction::Down,
            steps: 3,
        },
        Move {
            direction: Direction::Right,
            steps: 17,
        },
        Move {
            direction: Direction::Down,
            steps: 10,
        },
        Move {
            direction: Direction::Left,
            steps: 25,
        },
        Move {
            direction: Direction::Up,
            steps: 20,
        },
    ];

    let mut state = State::new(10);

    for r#move in &instructions {
        state.update(*r#move);
    }

    assert_eq!(state.num_visited_positions(), 36);
}
