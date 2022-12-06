use std::{io::BufRead, ops::RangeInclusive};

use crate::{get_day_input, Day, Part, Solver};

pub struct D4Solver;

impl Solver for D4Solver {
    type Solution = u64;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

pub struct Assignments {
    e1: RangeInclusive<u64>,
    e2: RangeInclusive<u64>,
}

fn parse_assignments(load_sample: bool) -> Vec<Assignments> {
    let file = get_day_input(Day::new(4), load_sample);
    let reader = std::io::BufReader::new(file);

    let mut assignments = Vec::new();

    for line in reader.lines().flatten() {
        let ranges = line.split(',').collect::<Vec<&str>>();
        let r1 = ranges[0]
            .split('-')
            .filter_map(|d| d.parse().ok())
            .collect::<Vec<_>>();
        let r2 = ranges[1]
            .split('-')
            .filter_map(|d| d.parse().ok())
            .collect::<Vec<_>>();

        let assignment = Assignments {
            e1: r1[0]..=r1[1],
            e2: r2[0]..=r2[1],
        };

        assignments.push(assignment);
    }

    assignments
}

fn solve_part_one() -> u64 {
    let assignments = parse_assignments(false);

    let mut shared_assignments = 0;

    for assignment in assignments {
        if assignment.e1.contains(assignment.e2.start())
            && assignment.e1.contains(assignment.e2.end())
            || assignment.e2.contains(assignment.e1.start())
                && assignment.e2.contains(assignment.e1.end())
        {
            shared_assignments += 1;
        }
    }

    shared_assignments
}

fn solve_part_two() -> u64 {
    let assignments = parse_assignments(false);

    let mut shared_assignments = 0;

    for assignment in assignments {
        if assignment.e1.contains(assignment.e2.start())
            || assignment.e1.contains(assignment.e2.end())
            || assignment.e2.contains(assignment.e1.start())
            || assignment.e2.contains(assignment.e1.end())
        {
            shared_assignments += 1;
        }
    }

    shared_assignments
}

#[test]
fn solve_sample_one() {
    let assignments = parse_assignments(true);

    let mut shared_assignments = 0;

    for assignment in assignments {
        if assignment.e1.contains(assignment.e2.start())
            && assignment.e1.contains(assignment.e2.end())
            || assignment.e2.contains(assignment.e1.start())
                && assignment.e2.contains(assignment.e1.end())
        {
            shared_assignments += 1;
        }
    }

    assert_eq!(2, shared_assignments);
}

#[test]
fn solve_sample_two() {
    let assignments = parse_assignments(true);

    let mut shared_assignments = 0;

    for assignment in assignments {
        if assignment.e1.contains(assignment.e2.start())
            || assignment.e1.contains(assignment.e2.end())
            || assignment.e2.contains(assignment.e1.start())
            || assignment.e2.contains(assignment.e1.end())
        {
            shared_assignments += 1;
        }
    }

    assert_eq!(4, shared_assignments);
}
