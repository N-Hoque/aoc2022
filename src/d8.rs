use std::io::BufRead;

use crate::{get_day_input, Day, Part, Solver};

pub struct D8Solver;

impl Solver for D8Solver {
    type Solution = u64;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

struct Forest {
    trees: Vec<Vec<u64>>,
    flipped_trees: Vec<Vec<u64>>,
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        let mut result = String::new();

        for row in &self.trees {
            for value in row {
                write!(&mut result, "{}", value).unwrap();
            }
            writeln!(&mut result).unwrap();
        }

        write!(f, "{}", result)
    }
}

impl Forest {
    pub fn new(trees: Vec<Vec<u64>>) -> Self {
        Self {
            trees: trees.clone(),
            flipped_trees: Self::transpose(trees),
        }
    }

    pub fn tree(&self, tree_row: usize, tree_col: usize) -> u64 {
        self.trees[tree_row][tree_col]
    }

    pub fn num_rows(&self) -> usize {
        self.trees.len()
    }

    pub fn num_cols(&self) -> usize {
        self.flipped_trees.len()
    }

    pub fn is_visible(&self, tree_row: usize, tree_col: usize) -> bool {
        if tree_row == 0
            || tree_row == self.num_rows() - 1
            || tree_col == 0
            || tree_col == self.num_cols() - 1
        {
            return true;
        }

        let tree = self.tree(tree_row, tree_col);

        let (left_row, right_row) = self.split_rows(tree_row, tree_col);
        let (left_col, right_col) = self.split_cols(tree_row, tree_col);

        let mut left_col = left_col.iter().rev();
        let mut right_col = right_col.iter().skip(1);
        let mut left_row = left_row.iter().rev();
        let mut right_row = right_row.iter().skip(1);

        left_col.all(|x| *x < tree)
            || right_col.all(|x| *x < tree)
            || left_row.all(|x| *x < tree)
            || right_row.all(|x| *x < tree)
    }

    pub fn scenic_score(&self, tree_row: usize, tree_col: usize) -> u64 {
        let tree = self.tree(tree_row, tree_col);

        let (left_row, right_row) = self.split_rows(tree_row, tree_col);
        let (left_col, right_col) = self.split_cols(tree_row, tree_col);

        let left_row = left_row.iter().rev();
        let right_row = right_row.iter().skip(1);
        let left_col = left_col.iter().rev();
        let right_col = right_col.iter().skip(1);

        let left_row_val = Self::find_score(tree, left_row);
        let right_row_val = Self::find_score(tree, right_row);
        let left_col_val = Self::find_score(tree, left_col);
        let right_col_val = Self::find_score(tree, right_col);

        left_col_val * left_row_val * right_col_val * right_row_val
    }

    fn find_score<'a>(tree: u64, values: impl Iterator<Item = &'a u64>) -> u64 {
        let mut current_score = 0;
        for value in values {
            current_score += 1;
            if *value >= tree {
                break;
            }
        }
        current_score
    }

    fn rows(&self) -> &Vec<Vec<u64>> {
        &self.trees
    }

    fn cols(&self) -> &Vec<Vec<u64>> {
        &self.flipped_trees
    }

    fn split_rows(&self, tree_row: usize, tree_col: usize) -> (&[u64], &[u64]) {
        self.rows()[tree_row].split_at(tree_col)
    }

    fn split_cols(&self, tree_row: usize, tree_col: usize) -> (&[u64], &[u64]) {
        self.cols()[tree_col].split_at(tree_row)
    }

    fn transpose(trees: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
        let v = trees;

        assert!(!v.is_empty());
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .filter_map(|n| n.next())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn parse_forest(load_sample: bool) -> Forest {
    let file = get_day_input(Day::new(8), load_sample);

    let reader = std::io::BufReader::new(file);

    let mut trees = Vec::new();

    for line in reader.lines().flatten() {
        let mut row = Vec::new();

        for char in line.chars() {
            let value = char.to_digit(10).unwrap() as u64;
            row.push(value);
        }

        trees.push(row);
    }

    Forest::new(trees)
}

fn solve_part_one() -> u64 {
    let forest = parse_forest(false);

    let num_rows = forest.num_rows();
    let num_cols = forest.num_cols();

    let mut num_trees_visible = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            if forest.is_visible(row, col) {
                num_trees_visible += 1;
            }
        }
    }

    num_trees_visible
}

fn solve_part_two() -> u64 {
    let forest = parse_forest(false);

    let num_rows = forest.num_rows();
    let num_cols = forest.num_cols();

    let mut max_scenic_score = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            let scenic_score = forest.scenic_score(row, col);
            if scenic_score >= max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

#[test]
fn solve_sample_one() {
    let trees = parse_forest(true);

    println!("{}", trees);

    let num_rows = trees.num_rows();
    let num_cols = trees.num_cols();

    let mut num_trees_visible = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            if trees.is_visible(row, col) {
                num_trees_visible += 1;
            }
        }
    }

    assert_eq!(num_trees_visible, 21);
}

#[test]
fn solve_sample_two() {
    let trees = parse_forest(true);

    let num_rows = trees.num_rows();
    let num_cols = trees.num_cols();

    let mut max_scenic_score = 0;

    for row in 0..num_rows {
        for col in 0..num_cols {
            let scenic_score = trees.scenic_score(row, col);
            if scenic_score >= max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    assert_eq!(max_scenic_score, 8);
}
