use std::fs::File;

pub(crate) mod d01;
pub(crate) mod d02;
pub(crate) mod d03;
pub(crate) mod d04;
pub(crate) mod d05;
pub(crate) mod d06;
pub(crate) mod d07;
pub(crate) mod d08;
pub(crate) mod d09;
pub(crate) mod d10;
pub(crate) mod d11;

#[must_use]
pub fn collect_all_solutions() -> Vec<String> {
    let mut solutions = Vec::new();

    let s1 = d01::Solver::solve(Part::One);
    let s2 = d01::Solver::solve(Part::Two);

    solutions.push(format!("1: {s1}, {s2}"));

    let s1 = d02::Solver::solve(Part::One);
    let s2 = d02::Solver::solve(Part::Two);

    solutions.push(format!("2: {s1}, {s2}"));

    let s1 = d03::Solver::solve(Part::One);
    let s2 = d03::Solver::solve(Part::Two);

    solutions.push(format!("3: {s1}, {s2}"));

    let s1 = d04::Solver::solve(Part::One);
    let s2 = d04::Solver::solve(Part::Two);

    solutions.push(format!("4: {s1}, {s2}"));

    let s1 = d05::Solver::solve(Part::One);
    let s2 = d05::Solver::solve(Part::Two);

    solutions.push(format!("5: {s1}, {s2}"));

    let s1 = d06::Solver::solve(Part::One);
    let s2 = d06::Solver::solve(Part::Two);

    solutions.push(format!("6: {s1}, {s2}"));

    let s1 = d07::Solver::solve(Part::One);
    let s2 = d07::Solver::solve(Part::Two);

    solutions.push(format!("7: {s1}, {s2}"));

    let s1 = d08::Solver::solve(Part::One);
    let s2 = d08::Solver::solve(Part::Two);

    solutions.push(format!("8: {s1}, {s2}"));

    let s1 = d09::Solver::solve(Part::One);
    let s2 = d09::Solver::solve(Part::Two);

    solutions.push(format!("9: {s1}, {s2}"));

    let s1 = d10::Solver::solve(Part::One);
    let s2 = d10::Solver::solve(Part::Two);

    solutions.push(format!("10: {s1}, \n{s2}"));

    let s1 = d11::D11Solver::solve(Part::One);
    let s2 = d11::D11Solver::solve(Part::Two);

    solutions.push(format!("11: {s1}, {s2}"));

    solutions
}

pub(crate) trait AOCSolver {
    type Solution;

    fn solve(part: Part) -> Self::Solution;
}

pub(crate) enum Part {
    One,
    Two,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct Day(u8);

impl Day {
    pub fn new(day: u8) -> Self {
        assert!(
            (1..=25).contains(&day),
            "day must be between 1 - 25. Got {} instead.",
            day
        );

        Self(day)
    }
}

pub(crate) fn get_day_input(day: Day, load_sample: bool) -> File {
    let path = if load_sample {
        format!("res/day_{}_sample.txt", day.0)
    } else {
        format!("res/day_{}.txt", day.0)
    };

    File::open(&path).unwrap_or_else(|_| panic!("Opening {path}"))
}
