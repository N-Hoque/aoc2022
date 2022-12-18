use std::io::Read;

use crate::{get_day_input, AOCSolver, Day, Part};

pub struct Solver;

impl AOCSolver for Solver {
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

fn find_magic_number(message: String, magic_length: usize) -> Option<usize> {
    let mut window = String::new();
    for (idx, char) in message.chars().enumerate() {
        if window.contains(char) {
            for _ in 0..=window.find(char).unwrap() {
                window.remove(0);
            }
        }
        window.push(char);

        if window.len() == magic_length {
            return Some(idx + 1);
        }
    }
    None
}

fn solve_part_one() -> usize {
    let message = load_message(false);

    if let Some(value) = find_magic_number(message, 4) {
        value
    } else {
        panic!("Oh dear we didn't find any sequence");
    }
}

fn solve_part_two() -> usize {
    let message = load_message(false);

    if let Some(value) = find_magic_number(message, 14) {
        value
    } else {
        panic!("Oh dear we didn't find any sequence");
    }
}
#[cfg(test)]
mod tests {
    use crate::d06::load_message;

    use super::find_magic_number;

    #[test]
    fn solve_sample_one_by_l4() {
        let message = load_message(true);

        if let Some(value) = find_magic_number(message, 4) {
            assert_eq!(7, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_one_by_l14() {
        let message = load_message(true);

        if let Some(value) = find_magic_number(message, 14) {
            assert_eq!(19, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_two_by_l4() {
        let message = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();

        if let Some(value) = find_magic_number(message, 4) {
            assert_eq!(5, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_two_by_l14() {
        let message = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();

        if let Some(value) = find_magic_number(message, 14) {
            assert_eq!(23, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_three_by_l4() {
        let message = "nppdvjthqldpwncqszvftbrmjlhg".to_string();

        if let Some(value) = find_magic_number(message, 4) {
            assert_eq!(6, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_three_by_l14() {
        let message = "nppdvjthqldpwncqszvftbrmjlhg".to_string();

        if let Some(value) = find_magic_number(message, 14) {
            assert_eq!(23, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_four_by_l4() {
        let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();

        if let Some(value) = find_magic_number(message, 4) {
            assert_eq!(10, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_four_by_l14() {
        let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();

        if let Some(value) = find_magic_number(message, 14) {
            assert_eq!(29, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_five_by_l4() {
        let message = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

        if let Some(value) = find_magic_number(message, 4) {
            assert_eq!(11, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }

    #[test]
    fn solve_sample_five_by_l14() {
        let message = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();

        if let Some(value) = find_magic_number(message, 14) {
            assert_eq!(26, value);
        } else {
            panic!("Oh dear we didn't find any sequence");
        }
    }
}
