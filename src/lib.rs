use std::{fs::File, io::BufRead, path::Path};

pub mod d1;

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

pub(crate) fn read_values<P: AsRef<Path>>(day: Day, load_sample: bool) -> Vec<u64> {
    let file = get_day_input(day, load_sample);

    let buffer = std::io::BufReader::new(file);

    let mut values = Vec::new();

    for line in buffer.lines().flatten() {
        let value = line.parse().expect("Parsing value");
        values.push(value);
    }

    values
}

pub(crate) fn read_chunked_values<P: AsRef<Path>>(day: Day, load_sample: bool) -> Vec<Vec<u64>> {
    let file = get_day_input(day, load_sample);

    let buffer = std::io::BufReader::new(file);

    let mut values = Vec::new();

    let mut chunk = Vec::new();

    for line in buffer.lines().flatten() {
        if line.is_empty() {
            values.push(chunk.clone());
            chunk.clear();
        } else {
            let value = line.parse().expect("Parsing value");
            chunk.push(value);
        }
    }

    values
}
