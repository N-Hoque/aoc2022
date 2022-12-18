use std::{
    collections::VecDeque,
    io::BufRead,
    iter::{IntoIterator, Iterator},
};

use crate::{get_day_input, AOCSolver, Day, Part};

pub struct Solver;

impl AOCSolver for Solver {
    type Solution = String;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(IntoIterator::into_iter).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(Iterator::next)
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
        })
        .collect()
}

#[derive(Debug)]
struct Instruction {
    source_stack: usize,
    target_stack: usize,
    move_amount: usize,
}

#[derive(Debug)]
struct Schedule {
    arrangement: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn parse_manifest(load_sample: bool) -> Schedule {
    let file = get_day_input(Day::new(5), load_sample);
    let reader = std::io::BufReader::new(file);

    let mut stacks = Vec::new();

    let mut stack = Vec::new();

    let mut section = true;

    let mut instructions = Vec::new();

    for line in reader.lines().flatten() {
        if line.is_empty() {
            section = false;
            continue;
        }
        if section {
            for char in line.chars().skip(1).step_by(4) {
                if char.is_ascii_uppercase() || char.is_ascii_whitespace() {
                    stack.push(char);
                }
            }
            if !stack.is_empty() {
                stacks.push(stack.clone());
                stack.clear();
            }
        } else {
            let mut values = Vec::new();

            let mut value = String::new();

            for char in line.chars().skip(5) {
                if char.is_ascii_digit() {
                    value.push(char);
                } else if char.is_whitespace() {
                    if value.is_empty() {
                        continue;
                    }
                    let res = value.parse().unwrap();
                    values.push(res);
                    value.clear();
                }
            }
            let instruction = Instruction {
                source_stack: values[1] - 1,
                target_stack: value.parse::<usize>().unwrap() - 1,
                move_amount: values[0],
            };
            instructions.push(instruction);
        }
    }

    let stacks = transpose(stacks.into_iter().rev().collect());

    Schedule {
        arrangement: stacks,
        instructions,
    }
}

fn solve_part_one() -> String {
    let mut schedule = parse_manifest(false);

    for Instruction {
        source_stack: source,
        target_stack: target,
        move_amount: amount,
    } in schedule.instructions
    {
        for _ in 0..amount {
            let last = schedule.arrangement[source].pop().expect("Get crate");
            schedule.arrangement[target].push(last);
        }
    }

    let mut top_row = String::new();
    for stack in schedule.arrangement {
        top_row.push(*stack.last().unwrap());
    }
    top_row
}

fn solve_part_two() -> String {
    let mut schedule = parse_manifest(false);

    for Instruction {
        source_stack: source,
        target_stack: target,
        move_amount: amount,
    } in schedule.instructions
    {
        let mut queue = VecDeque::new();

        for _ in 0..amount {
            let last = schedule.arrangement[source].pop().expect("Get crate");
            queue.push_front(last);
        }

        schedule.arrangement[target].extend(&queue);
    }

    let mut top_row = String::new();
    for stack in schedule.arrangement {
        top_row.push(*stack.last().unwrap());
    }
    top_row
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::d05::{parse_manifest, Instruction};

    #[test]
    fn solve_sample_one() {
        let mut schedule = parse_manifest(true);

        for Instruction {
            source_stack: source,
            target_stack: target,
            move_amount: amount,
        } in schedule.instructions
        {
            for _ in 0..amount {
                let last = schedule.arrangement[source].pop().expect("Get crate");
                schedule.arrangement[target].push(last);
            }
        }

        let mut top_row = String::new();
        for stack in schedule.arrangement {
            top_row.push(*stack.last().unwrap());
        }

        assert_eq!("CMZ", top_row);
    }

    #[test]
    fn solve_sample_two() {
        let mut schedule = parse_manifest(true);

        for Instruction {
            source_stack: source,
            target_stack: target,
            move_amount: amount,
        } in schedule.instructions
        {
            let mut queue = VecDeque::new();

            for _ in 0..amount {
                let last = schedule.arrangement[source].pop().expect("Get crate");
                queue.push_front(last);
            }

            schedule.arrangement[target].extend(&queue);
        }

        let mut top_row = String::new();
        for stack in schedule.arrangement {
            top_row.push(*stack.last().unwrap());
        }

        assert_eq!("MCD", top_row);
    }
}
