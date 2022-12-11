use std::fs::File;

pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;

pub trait Solver {
    type Solution;

    fn solve(part: Part) -> Self::Solution;
}

pub enum Part {
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

    File::open(&path).unwrap_or_else(|_| panic!("Opening {}", path))
}
