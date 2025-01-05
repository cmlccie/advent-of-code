use crate::shared::inputs::get_input;
use nalgebra::{matrix, vector};
use regex::Regex;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 13: Claw Contraption
-------------------------------------------------------------------------------------------------*/

const F64_TOLERANCE: f64 = 1e-4;

pub fn part1(input: &str) -> Option<String> {
    let claw_machines = parse_input(input);

    let minimum_token_count = claw_machines
        .iter()
        .filter_map(|claw_machine| claw_machine.solve())
        .map(|(a, b)| 3 * a + b)
        .sum::<u64>();

    Some(minimum_token_count.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let claw_machines = parse_input(input);

    let updated_measurements = claw_machines
        .iter()
        .map(|claw_machine| ClawMachine {
            x: claw_machine.x + 10000000000000,
            y: claw_machine.y + 10000000000000,
            ..*claw_machine
        })
        .collect::<Vec<_>>();

    let minimum_token_count = updated_measurements
        .iter()
        .filter_map(|claw_machine| claw_machine.solve())
        .map(|(a, b)| 3 * a + b)
        .sum::<u64>();

    Some(minimum_token_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let claw_machine_regex = Regex::new(
        r#"(?x)
        Button\sA:\sX\+(?P<ax>\d+),\sY\+(?P<ay>\d+)\n
        Button\sB:\sX\+(?P<bx>\d+),\sY\+(?P<by>\d+)\n
        Prize:\sX=(?P<x>\d+),\sY=(?P<y>\d+)
        "#,
    )
    .unwrap();

    claw_machine_regex
        .captures_iter(input)
        .map(|cap| ClawMachine {
            ax: cap["ax"].parse().unwrap(),
            bx: cap["bx"].parse().unwrap(),
            x: cap["x"].parse().unwrap(),
            ay: cap["ay"].parse().unwrap(),
            by: cap["by"].parse().unwrap(),
            y: cap["y"].parse().unwrap(),
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct ClawMachine {
    ax: u64,
    bx: u64,
    x: u64,
    ay: u64,
    by: u64,
    y: u64,
}

impl ClawMachine {
    fn solve(&self) -> Option<(u64, u64)> {
        let coefficients = matrix![
            self.ax as f64, self.bx as f64;
            self.ay as f64, self.by as f64
        ];
        let constants = vector![self.x as f64, self.y as f64];
        let solution = coefficients.lu().solve(&constants)?;

        // Valid solutions must be integers
        if solution.iter().any(|f| f.fract().abs() < F64_TOLERANCE) {
            Some((solution[0].round() as u64, solution[1].round() as u64))
        } else {
            None
        }
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 13: Claw Contraption")]
pub enum Args {
    Part1 { input: PathBuf },
    Part2 { input: PathBuf },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 { input } => part1(&get_input(&input)),
        Args::Part2 { input } => part2(&get_input(&input)),
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::answers::get_answer;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(&get_input("../data/day13/example.txt")),
            get_answer("../data/day13/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day13/input.txt")),
            get_answer("../data/day13/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day13/input.txt")),
            get_answer("../data/day13/input-part2-answer.txt")
        );
    }
}
