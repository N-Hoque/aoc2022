use std::io::BufRead;

use crate::{get_day_input, Day, Part, Solver};

pub struct D1Solver;

impl Solver for D1Solver {
    type Solution = u64;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

fn summarise_elves(load_sample: bool) -> Vec<u64> {
    let file = get_day_input(Day::new(1), load_sample);
    let reader = std::io::BufReader::new(file);

    let mut summed_values = Vec::new();

    let mut sum = 0;

    for line in reader.lines().flatten() {
        if line.is_empty() {
            summed_values.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u64>().expect("Parsing value");
        }
    }

    summed_values.push(sum);

    summed_values
}

fn solve_part_one() -> u64 {
    let summed_values = summarise_elves(false);

    *summed_values.iter().max().unwrap()
}

fn solve_part_two() -> u64 {
    let mut summed_values = summarise_elves(false);

    summed_values.sort_unstable_by(|x, y| y.cmp(x));

    summed_values[0] + summed_values[1] + summed_values[2]
}

#[test]
fn solve_sample_one() {
    let summed_values = summarise_elves(true);

    assert_eq!(Some(24000), summed_values.iter().max().copied());
}

#[test]
fn solve_sample_two() {
    let mut summed_values = summarise_elves(true);

    summed_values.sort_unstable_by(|x, y| y.cmp(x));

    let max_three_sum = summed_values[0] + summed_values[1] + summed_values[2];

    assert_eq!(45000, max_three_sum);
}
