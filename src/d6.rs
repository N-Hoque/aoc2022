use std::io::Read;

use crate::{get_day_input, Day, Part, Solver};

pub struct D6Solver;

impl Solver for D6Solver {
    type Solution = usize;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

fn load_message(load_sample: bool) -> String {
    let mut file = get_day_input(Day::new(6), load_sample);

    let mut res = String::new();

    file.read_to_string(&mut res).expect("Reading file");

    res
}

fn solve_part_one() -> usize {
    let message = load_message(false);

    let mut window = String::new();

    for (idx, char) in message.chars().enumerate() {
        if window.contains(char) {
            for _ in 0..(window.find(char).unwrap() + 1) {
                window.remove(0);
            }
        }
        window.push(char);

        if window.len() == 4 {
            return idx + 1;
        }
    }

    panic!("Oh dear we didn't find any sequence");
}

fn solve_part_two() -> usize {
    let message = load_message(false);

    let mut window = String::new();

    for (idx, char) in message.chars().enumerate() {
        if window.contains(char) {
            for _ in 0..(window.find(char).unwrap() + 1) {
                window.remove(0);
            }
        }
        window.push(char);

        if window.len() == 14 {
            return idx + 1;
        }
    }

    panic!("Oh dear we didn't find any sequence");
}

#[test]
fn solve_sample_one() {
    let message = load_message(true);

    let mut window = String::new();

    let mut result = 0;

    for (idx, char) in message.chars().enumerate() {
        if window.contains(char) {
            let first_char = window.find(char);
            for _ in 0..(first_char.unwrap() + 1) {
                window.remove(0);
            }
        }
        window.push(char);

        if window.len() == 4 {
            result = idx + 1;
            break;
        }
    }

    assert_eq!(7, result);
}

#[test]
fn solve_sample_two() {
    let message = load_message(true);

    let mut window = String::new();

    let mut result = 0;

    for (idx, char) in message.chars().enumerate() {
        if window.contains(char) {
            let first_char = window.find(char);
            for _ in 0..(first_char.unwrap() + 1) {
                window.remove(0);
            }
        }
        window.push(char);

        if window.len() == 14 {
            result = idx + 1;
            break;
        }
    }

    assert_eq!(19, result);
}
