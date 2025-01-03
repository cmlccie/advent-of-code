use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/*-------------------------------------------------------------------------------------------------
  Day 11: Plutonian Pebbles
-------------------------------------------------------------------------------------------------*/

fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let stones = parse_input_file(input);
    let stone_count = blinks(&stones, 25);

    Some(stone_count.to_string())
}

fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let stones = parse_input_file(input);
    let stone_count = blinks(&stones, 75);

    Some(stone_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Stone = u64;
type StoneCount = usize;
type BlinkCount = u8;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Vec<Stone> {
    read_to_string(input)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn blinks(stones: &[Stone], count: BlinkCount) -> StoneCount {
    let mut cache = Cache::new();
    stones
        .iter()
        .map(|s| cache.recursive_blink(s, count - 1))
        .sum()
}

fn transform(stone: &Stone) -> Vec<Stone> {
    let stone_text = stone.to_string();
    if stone == &0 {
        vec![1]
    } else if stone_text.len() % 2 == 0 {
        let (left, right) = stone_text.split_at(stone_text.len() / 2);
        vec![left.parse().unwrap(), right.parse().unwrap()]
    } else {
        vec![stone * 2024]
    }
}

/*-----------------------------------------------------------------------------
  Recursive Blink Cache
-----------------------------------------------------------------------------*/

struct Cache {
    stone_cache: HashMap<Stone, Vec<Stone>>,
    results_cache: HashMap<(Stone, BlinkCount), usize>,
}

impl Cache {
    fn new() -> Self {
        Self {
            stone_cache: HashMap::new(),
            results_cache: HashMap::new(),
        }
    }

    fn recursive_blink(&mut self, stone: &Stone, blinks: BlinkCount) -> StoneCount {
        // Check for cache hits
        if let Some(result) = self.results_cache.get(&(*stone, blinks)) {
            return *result;
        };

        let new_stones = self
            .stone_cache
            .entry(*stone)
            .or_insert_with(|| transform(stone))
            .clone();

        // Base case
        if blinks == 0 {
            return new_stones.len();
        };

        // Recursive case
        let result = new_stones
            .iter()
            .map(|s| self.recursive_blink(s, blinks - 1))
            .sum();

        // Cache the result
        self.results_cache.insert((*stone, blinks), result);

        result
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 11: Plutonian Pebbles")]
pub enum Args {
    Part1 { input: PathBuf },
    Part2 { input: PathBuf },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 { input } => part1(&input),
        Args::Part2 { input } => part2(&input),
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1("../data/day11/example.txt"),
            solution("../data/day11/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day11/input.txt"),
            solution("../data/day11/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day11/input.txt"),
            solution("../data/day11/input-part2-answer.txt")
        );
    }
}
