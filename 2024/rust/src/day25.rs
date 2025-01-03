use core::panic;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use itertools::Itertools;

/*-------------------------------------------------------------------------------------------------
  Day 25: Code Chronicle
-------------------------------------------------------------------------------------------------*/

fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let (locks, keys) = parse_input_file(input);

    let lock_key_pair_count = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5))
        .count();

    Some(lock_key_pair_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Schematic = [u8; 5];
type Lock = Schematic;
type Locks = Vec<Lock>;

type Key = Schematic;
type Keys = Vec<Key>;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> (Locks, Keys) {
    let input = read_to_string(input).unwrap();
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        let first_line = lines.next().unwrap();
        let (lock, key) = match first_line {
            "#####" => (true, false),
            "....." => (false, true),
            _ => panic!("Invalid first line: {first_line}"),
        };

        let mut value: Schematic = [0; 5];
        for _ in 0..5 {
            let line = lines.next().unwrap();
            assert_eq!(line.len(), 5);
            for (i, c) in line.chars().enumerate() {
                value[i] += match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Invalid character: {c}"),
                };
            }
        }

        let end_line = lines.next().unwrap();
        match (lock, key) {
            (true, false) => {
                assert_eq!(end_line, ".....");
                locks.push(value);
            }
            (false, true) => {
                assert_eq!(end_line, "#####");
                keys.push(value);
            }
            _ => panic!("Invalid lock/key flags: {lock}/{key}"),
        }

        let _ = lines.next(); // Skip blank line
    }

    (locks, keys)
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 25: Code Chronicle")]
pub enum Args {
    Part1 { input: PathBuf },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 { input } => part1(&input),
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_parse_input_file() {
        let (locks, keys) = parse_input_file("../data/day25/example.txt");

        assert_eq!(locks, vec![[0, 5, 3, 4, 3], [1, 2, 0, 5, 3]]);
        assert_eq!(
            keys,
            vec![[5, 0, 2, 1, 3], [4, 3, 4, 0, 2], [3, 0, 2, 0, 1]]
        );
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1("../data/day25/example.txt"),
            solution("../data/day25/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day25/input.txt"),
            solution("../data/day25/input-part1-answer.txt")
        );
    }
}
