use std::io::BufRead;

use crate::{get_day_input, AOCSolver, Day, Part};

const SCREEN_WIDTH: usize = 40;

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

type Instruction = Option<i64>;

#[derive(Debug)]
struct CPU {
    register: i64,
    clock: usize,
    signal_strength_buffer: Vec<i64>,
    screen_buffer: Vec<char>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            register: 1,
            clock: 0,
            signal_strength_buffer: Vec::new(),
            screen_buffer: Vec::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        let (tick_counter, add_amount) = if let Some(x) = instruction {
            (2, x)
        } else {
            (1, 0)
        };

        for tick in 0..tick_counter {
            self.screen_buffer.push({
                if ((self.register - 1) as usize..=(self.register + 1) as usize)
                    .contains(&(self.clock % SCREEN_WIDTH))
                {
                    '#'
                } else {
                    '.'
                }
            });
            self.tick();
            if (self.clock + 20) % SCREEN_WIDTH == 0 {
                self.signal_strength_buffer
                    .push(self.clock as i64 * self.register);
            }

            if tick == 1 {
                self.register += add_amount;
            }
        }
    }

    pub fn calculate_signal_strength_sum(&self) -> i64 {
        self.signal_strength_buffer.iter().sum()
    }

    pub fn display_screen_buffer(&self) -> String {
        let mut screen = String::new();

        for (idx, element) in self.screen_buffer.iter().enumerate() {
            screen.push(*element);
            if (idx + 1) % SCREEN_WIDTH == 0 {
                screen += "\n";
            }
        }

        screen
    }

    fn tick(&mut self) {
        self.clock += 1;
    }
}

fn parse_instructions(load_sample: bool) -> Vec<Instruction> {
    let file = get_day_input(Day::new(10), load_sample);

    let reader = std::io::BufReader::new(file);

    let mut instructions = Vec::new();

    for line in reader.lines().flatten() {
        if line == "noop" {
            instructions.push(None)
        } else {
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

            let add_amount = parts[1].parse().unwrap();
            instructions.push(Some(add_amount));
        }
    }

    instructions
}

fn solve_part_one() -> String {
    let instructions = parse_instructions(false);

    let mut cpu = CPU::new();

    for instruction in instructions {
        cpu.execute(instruction);
    }

    cpu.calculate_signal_strength_sum().to_string()
}

fn solve_part_two() -> String {
    let instructions = parse_instructions(false);

    let mut cpu = CPU::new();

    for instruction in instructions {
        cpu.execute(instruction);
    }

    cpu.display_screen_buffer()
}

#[cfg(test)]
mod tests {
    use super::{parse_instructions, CPU};

    #[test]
    fn solve_sample_zero() {
        let instructions = vec![None, Some(3), Some(-5)];

        let mut cpu = CPU::new();

        for instruction in instructions {
            cpu.execute(instruction);
            println!("{:?}", cpu);
        }
    }

    #[test]
    fn solve_sample_one() {
        let instructions = parse_instructions(true);

        let mut cpu = CPU::new();

        for instruction in instructions {
            cpu.execute(instruction);
        }

        assert_eq!(13140, cpu.calculate_signal_strength_sum())
    }

    #[test]
    fn solve_sample_two() {
        let instructions = parse_instructions(true);

        let mut cpu = CPU::new();

        for instruction in instructions {
            cpu.execute(instruction);
        }

        let display = cpu.display_screen_buffer();

        let expected = r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(expected, display)
    }
}
