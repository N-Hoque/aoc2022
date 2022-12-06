use std::{collections::HashSet, io::BufRead};

use crate::{get_day_input, Day, Part, Solver};

pub struct D3Solver;

impl Solver for D3Solver {
    type Solution = u64;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

fn parse_rucksacks(load_sample: bool) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    let file = get_day_input(Day::new(3), load_sample);
    let reader = std::io::BufReader::new(file);

    let mut compartments = Vec::new();

    let (mut c1, mut c2) = (HashSet::new(), HashSet::new());

    for line in reader.lines().flatten() {
        let (a, b) = line.split_at(line.len() / 2);
        for ch in a.chars() {
            c1.insert(if ch.is_ascii_uppercase() {
                ch as u8 - 65 + 27
            } else {
                ch as u8 - 97 + 1
            });
        }
        for ch in b.chars() {
            c2.insert(if ch.is_ascii_uppercase() {
                ch as u8 - 65 + 27
            } else {
                ch as u8 - 97 + 1
            });
        }
        compartments.push((c1.clone(), c2.clone()));
        c1.clear();
        c2.clear();
    }

    compartments
}

fn solve_part_one() -> u64 {
    let compartments = parse_rucksacks(false);

    compartments.into_iter().fold(0, |acc, (c1, c2)| {
        let cs = c1.intersection(&c2).collect::<Vec<_>>();
        acc + *cs[0] as u64
    })
}

fn solve_part_two() -> u64 {
    let compartments = parse_rucksacks(false);

    let mut grouped_compartments = Vec::new();

    let mut group = Vec::new();

    for (idx, (c1, c2)) in compartments.into_iter().enumerate() {
        group.push((c1, c2));
        if idx % 3 == 2 {
            grouped_compartments.push(group.clone());
            group.clear();
        }
    }

    let mut summed_groups = 0;

    for group in grouped_compartments {
        let mut badges: HashSet<u8> = HashSet::new();

        for (c1, c2) in group {
            let cs = c1.union(&c2).copied().collect::<HashSet<_>>();
            if badges.is_empty() {
                badges = cs;
            } else {
                badges = badges.intersection(&cs).copied().collect::<HashSet<_>>();
            }
        }

        summed_groups += badges.into_iter().map(|x| x as u64).sum::<u64>();
    }

    summed_groups
}

#[test]
fn test() {
    let compartments = parse_rucksacks(true);
    println!("{:?}", compartments);
}

#[test]
fn solve_sample_one() {
    let compartments = parse_rucksacks(true);

    let summed_priorities = compartments.into_iter().fold(0, |acc, (c1, c2)| {
        let cs = c1.intersection(&c2).collect::<Vec<_>>();
        acc + *cs[0] as u64
    });

    assert_eq!(157, summed_priorities);
}

#[test]
fn solve_sample_two() {
    let compartments = parse_rucksacks(true);

    let mut grouped_compartments = Vec::new();

    let mut group = Vec::new();

    for (idx, (c1, c2)) in compartments.into_iter().enumerate() {
        group.push((c1, c2));
        if idx % 3 == 2 {
            grouped_compartments.push(group.clone());
            group.clear();
        }
    }

    let mut summed_groups = 0;

    for group in grouped_compartments {
        let mut badges: HashSet<u8> = HashSet::new();

        for (c1, c2) in group {
            let cs = c1.union(&c2).copied().collect::<HashSet<_>>();
            if badges.is_empty() {
                badges = cs;
            } else {
                badges = badges.intersection(&cs).copied().collect::<HashSet<_>>();
            }
        }

        summed_groups += badges.into_iter().map(|x| x as u64).sum::<u64>();
    }

    assert_eq!(70, summed_groups);
}
