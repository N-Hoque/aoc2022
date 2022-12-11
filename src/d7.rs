use std::{collections::HashMap, io::BufRead};

use petgraph::{prelude::DiGraph, stable_graph::NodeIndex, visit::Dfs, Graph};

use crate::{get_day_input, Day, Part, Solver};

pub struct D7Solver;

impl Solver for D7Solver {
    type Solution = u64;

    fn solve(part: Part) -> Self::Solution {
        if let Part::One = part {
            solve_part_one()
        } else {
            solve_part_two()
        }
    }
}

#[derive(Debug, Clone)]
enum Command {
    List,
    Change(String),
}

#[derive(Debug, Clone)]
enum Handle {
    File(String, u64),
    Directory(String),
}

#[derive(Debug, Clone)]
enum Statement {
    Command(Command),
    Handle(Handle),
}

fn parse_input(data: &[String]) -> Vec<Statement> {
    let mut statements = Vec::new();

    for line in data {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        if parts[0] == "$" {
            if parts[1] == "ls" {
                statements.push(Statement::Command(Command::List));
            } else {
                statements.push(Statement::Command(Command::Change(parts[2].to_owned())));
            }
        } else if parts[0] == "dir" {
            statements.push(Statement::Handle(Handle::Directory(parts[1].to_owned())));
        } else {
            statements.push(Statement::Handle(Handle::File(
                parts[1].to_owned(),
                parts[0].parse().unwrap(),
            )));
        }
    }

    statements
}

fn parse_filesystem(load_sample: bool) -> DiGraph<String, u64> {
    let file = get_day_input(Day::new(7), load_sample);

    let reader = std::io::BufReader::new(file);

    let data = reader.lines().flatten().collect::<Vec<String>>();

    let mut filesystem = DiGraph::new();

    let mut dir_stack = Vec::new();

    let mut current_dir = None;

    let statements = parse_input(&data);

    for statement in statements {
        match statement {
            Statement::Command(c) => match c {
                Command::List => continue,
                Command::Change(dir) => {
                    current_dir = if dir == ".." {
                        let _d = dir_stack.pop();
                        dir_stack.last().copied()
                    } else if dir == "/" {
                        let root_node = filesystem.add_node(dir);
                        dir_stack.push(root_node);
                        Some(root_node)
                    } else if let Some(neighbor) = filesystem
                        .neighbors(current_dir.unwrap())
                        .find(|n| filesystem[*n] == *dir)
                    {
                        dir_stack.push(neighbor);
                        Some(neighbor)
                    } else {
                        panic!(
                            "{} is not reachable from {}",
                            dir,
                            filesystem[current_dir.unwrap()]
                        );
                    };
                }
            },
            Statement::Handle(h) => match h {
                Handle::File(name, size) => {
                    let new_node = filesystem.add_node(name);
                    filesystem.add_edge(current_dir.unwrap(), new_node, size);
                }
                Handle::Directory(name) => {
                    let new_node = filesystem.add_node(name);
                    filesystem.add_edge(current_dir.unwrap(), new_node, 0);
                }
            },
        }
    }

    filesystem
}

fn update_filesystem(filesystem: DiGraph<String, u64>) -> DiGraph<String, u64> {
    let mut filesystem = filesystem;

    let root_node = filesystem.node_indices().next().unwrap();

    let mut dfs = Dfs::new(&filesystem, root_node);

    let mut zero_size_dir_stack = Vec::new();

    while let Some(node) = dfs.next(&filesystem) {
        let neighbors = filesystem.neighbors(node);

        for neighbor in neighbors {
            if let Some(edge) = filesystem.find_edge(node, neighbor) {
                let weight = filesystem[edge];
                if weight == 0 {
                    zero_size_dir_stack.push((node, neighbor));
                }
            }
        }
    }

    while let Some((node, zero_node)) = zero_size_dir_stack.pop() {
        let neighbors = filesystem.neighbors(zero_node);

        let mut dir_size = 0;

        for neighbor in neighbors {
            if let Some(edge) = filesystem.find_edge(zero_node, neighbor) {
                let weight = filesystem[edge];
                dir_size += weight;
            }
        }

        filesystem.update_edge(node, zero_node, dir_size);
    }

    filesystem
}

fn find_directory_sizes(
    filesystem: Graph<String, u64>,
    root: NodeIndex,
) -> HashMap<NodeIndex, u64> {
    let mut dfs = Dfs::new(&filesystem, root);
    let mut dirs = HashMap::new();
    while let Some(node) = dfs.next(&filesystem) {
        let neighbor_edges = filesystem.edges(node).collect::<Vec<_>>();
        if neighbor_edges.is_empty() {
            continue;
        }

        let dir_size = neighbor_edges.into_iter().map(|e| *e.weight()).sum();
        dirs.insert(node, dir_size);
    }
    dirs
}

fn find_min_dir_size(dirs: HashMap<NodeIndex, u64>, root: NodeIndex) -> u64 {
    const MAX_SPACE: u64 = 70000000;
    const MIN_SPACE: u64 = 30000000;

    let filesystem_amount_left = MAX_SPACE - dirs[&root];
    let mut dirs_as_vec = dirs.into_iter().collect::<Vec<(_, _)>>();
    dirs_as_vec.sort_by(|(_, s1), (_, s2)| s1.cmp(s2));

    let min_space = dirs_as_vec.into_iter().find_map(|(_, size)| {
        if filesystem_amount_left + size >= MIN_SPACE {
            Some(size)
        } else {
            None
        }
    });

    min_space.unwrap()
}

fn solve_part_one() -> u64 {
    let filesystem = parse_filesystem(false);
    let filesystem = update_filesystem(filesystem);

    let root = filesystem.node_indices().next().unwrap();

    let dirs = find_directory_sizes(filesystem, root);

    dirs.values().filter(|w| **w <= 100000).sum::<u64>()
}

fn solve_part_two() -> u64 {
    let filesystem = parse_filesystem(false);
    let filesystem = update_filesystem(filesystem);

    let root = filesystem.node_indices().next().unwrap();

    let dirs = find_directory_sizes(filesystem, root);

    find_min_dir_size(dirs, root)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use petgraph::dot::Dot;

    use super::{find_directory_sizes, find_min_dir_size, parse_filesystem, update_filesystem};

    #[test]
    fn read_filesystem() {
        let filesystem = parse_filesystem(true);

        let mut test_file = std::fs::File::create("res/day_7_sample.dot").unwrap();

        write!(&mut test_file, "{}", Dot::new(&filesystem)).unwrap();

        let filesystem = update_filesystem(filesystem);

        let mut test_file = std::fs::File::create("res/day_7_sample_updated.dot").unwrap();

        write!(&mut test_file, "{}", Dot::new(&filesystem)).unwrap();
    }

    #[test]
    fn solve_sample_one() {
        let filesystem = parse_filesystem(true);
        let filesystem = update_filesystem(filesystem);

        let root = filesystem.node_indices().next().unwrap();

        let dirs = find_directory_sizes(filesystem, root);

        let total_size = dirs.values().filter(|w| **w <= 100000).sum::<u64>();

        assert_eq!(total_size, 95437);
    }

    #[test]
    fn solve_sample_two() {
        let filesystem = parse_filesystem(true);
        let filesystem = update_filesystem(filesystem);

        let root = filesystem.node_indices().next().unwrap();

        let dirs = find_directory_sizes(filesystem, root);

        let min_size = find_min_dir_size(dirs, root);

        assert_eq!(min_size, 24933642);
    }
}
