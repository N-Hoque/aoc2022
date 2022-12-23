use std::{collections::VecDeque, io::BufRead};

use crate::{get_day_input, AOCSolver, Day, Part};

pub struct D11Solver;

impl AOCSolver for D11Solver {
    type Solution = u64;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Original,
    Number(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpType {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    rhs: Value,
    op_type: OpType,
}

type MonkeyID = usize;

#[derive(Debug, Clone)]
struct Monkey {
    id: MonkeyID,
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    destination: (MonkeyID, MonkeyID),
    inspection_count: u64,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            id: MonkeyID::MAX,
            items: Default::default(),
            operation: Operation {
                rhs: Value::Number(u64::MIN),
                op_type: OpType::Add,
            },
            test: Default::default(),
            destination: (MonkeyID::MAX, MonkeyID::MAX),
            inspection_count: 0,
        }
    }
}

#[derive(Debug)]
struct Round {
    num_rounds: usize,
    monkeys: Vec<Monkey>,
    num_monkeys: usize,
    current_monkey_index: usize,
    super_factor: Option<u64>,
}

impl Round {
    pub fn new(monkeys: Vec<Monkey>, num_rounds: usize, enable_worry_factor: bool) -> Self {
        Self {
            num_rounds,
            super_factor: if enable_worry_factor {
                Some(Self::find_common_test_factor(&monkeys))
            } else {
                None
            },
            num_monkeys: monkeys.len(),
            current_monkey_index: 0,
            monkeys,
        }
    }

    pub fn run(&mut self) {
        for _ in 0..self.num_rounds {
            for _ in 0..self.num_monkeys {
                let (true_dest, false_dest) = self.find_items_from_current_monkey();
                self.update_destination(true_dest);
                self.update_destination(false_dest);
                self.next();
            }
        }
    }

    fn find_common_test_factor(monkeys: &[Monkey]) -> u64 {
        monkeys.iter().fold(1, |acc, m| acc * m.test)
    }

    fn find_items_from_current_monkey(&mut self) -> ((MonkeyID, Vec<u64>), (MonkeyID, Vec<u64>)) {
        let current_monkey = self
            .monkeys
            .get_mut(self.current_monkey_index)
            .unwrap_or_else(|| {
                panic!(
                    "Get reference to monkey with ID: {}",
                    self.current_monkey_index
                )
            });

        let mut true_destination = (current_monkey.destination.0, Vec::new());
        let mut false_destination = (current_monkey.destination.1, Vec::new());

        while let Some(item) = current_monkey.items.pop_front() {
            let worry_level = match (
                current_monkey.operation.op_type,
                current_monkey.operation.rhs,
            ) {
                (OpType::Add, Value::Original) => item + item,
                (OpType::Add, Value::Number(rhs)) => item + rhs,
                (OpType::Mul, Value::Original) => item * item,
                (OpType::Mul, Value::Number(rhs)) => item * rhs,
            };

            let worry_level = if let Some(super_factor) = self.super_factor {
                worry_level % super_factor
            } else {
                worry_level / 3
            };
            current_monkey.inspection_count += 1;

            if worry_level % current_monkey.test == 0 {
                true_destination.1.push(worry_level);
            } else {
                false_destination.1.push(worry_level);
            }
        }

        (true_destination, false_destination)
    }

    fn update_destination(&mut self, destination: (MonkeyID, Vec<u64>)) {
        let monkey = self
            .monkeys
            .get_mut(destination.0)
            .unwrap_or_else(|| panic!("Get reference to monkey with ID: {}", destination.0));

        monkey.items.extend(destination.1.into_iter());
    }

    fn next(&mut self) {
        self.current_monkey_index = (self.current_monkey_index + 1) % self.num_monkeys;
    }
}

fn parse_monkeys(load_sample: bool) -> Vec<Monkey> {
    let file = get_day_input(Day::new(11), load_sample);

    let reader = std::io::BufReader::new(file);

    let mut monkeys = Vec::new();

    let mut monkey = Monkey::default();

    for line in reader.lines().flatten() {
        if line.is_empty() {
            monkeys.push(monkey);
            monkey = Monkey::default();
        } else {
            let line = line.trim().replace([',', ':'], "");
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

            if parts[0].starts_with('M') {
                monkey.id = parts[1].parse().unwrap();
            } else if parts[0].starts_with('S') {
                let values = parts[2..].iter().filter_map(|x| x.parse().ok()).collect();
                monkey.items = values;
            } else if parts[0].starts_with('O') {
                let op = if parts[4] == "*" {
                    OpType::Mul
                } else {
                    OpType::Add
                };
                let value = if parts[5] == "old" {
                    Value::Original
                } else {
                    Value::Number(parts[5].parse().unwrap())
                };
                monkey.operation = Operation {
                    rhs: value,
                    op_type: op,
                }
            } else if parts[0].starts_with('T') {
                let test = parts[3].parse().unwrap();
                monkey.test = test;
            } else if parts[1] == "true" {
                monkey.destination.0 = parts[5].parse().unwrap();
            } else {
                monkey.destination.1 = parts[5].parse().unwrap();
            }
        }
    }

    monkeys.push(monkey);

    monkeys
}

fn solve_part_one() -> u64 {
    let monkeys = parse_monkeys(false);

    let mut round = Round::new(monkeys, 20, false);

    round.run();

    let mut monkeys = round.monkeys.clone();

    monkeys.sort_by(|m1, m2| m2.inspection_count.cmp(&m1.inspection_count));

    monkeys
        .into_iter()
        .take(2)
        .fold(1, |acc, m| acc * m.inspection_count)
}

fn solve_part_two() -> u64 {
    let monkeys = parse_monkeys(false);

    let mut round = Round::new(monkeys, 10000, true);

    round.run();

    let mut monkeys = round.monkeys.clone();

    monkeys.sort_by(|m1, m2| m2.inspection_count.cmp(&m1.inspection_count));

    monkeys
        .into_iter()
        .take(2)
        .fold(1, |acc, m| acc * m.inspection_count)
}

#[cfg(test)]
mod tests {
    use super::{parse_monkeys, Round};

    #[test]
    fn solve_sample_one() {
        let monkeys = parse_monkeys(true);

        let mut round = Round::new(monkeys, 20, false);

        round.run();

        let mut monkeys = round.monkeys.clone();

        monkeys.sort_by(|m1, m2| m2.inspection_count.cmp(&m1.inspection_count));

        let monkey_business = monkeys
            .into_iter()
            .take(2)
            .fold(1, |acc, m| acc * m.inspection_count);

        assert_eq!(10605, monkey_business);
    }

    #[test]
    fn solve_sample_two() {
        let monkeys = parse_monkeys(true);

        let mut round = Round::new(monkeys, 10000, true);

        round.run();

        let mut monkeys = round.monkeys;

        println!("{:?}", monkeys);

        assert_eq!(52166, monkeys[0].inspection_count);
        assert_eq!(47830, monkeys[1].inspection_count);
        assert_eq!(1938, monkeys[2].inspection_count);
        assert_eq!(52013, monkeys[3].inspection_count);

        monkeys.sort_by(|m1, m2| m2.inspection_count.cmp(&m1.inspection_count));

        let monkey_business = monkeys
            .into_iter()
            .take(2)
            .fold(1, |acc, m| acc * m.inspection_count);

        assert_eq!(2713310158, monkey_business);
    }
}
