use crate::get_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 8: Resonant Collinearity
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let map = parse_input(input);

    let anti_nodes: HashSet<Coordinate> = get_antenna_pairs(&map.antennas)
        .iter()
        .flat_map(project_anti_nodes)
        .filter(|coordinate| filter_off_map_coordinates(&map, coordinate))
        .collect();

    Some(anti_nodes.len().to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let map = parse_input(input);

    let anti_nodes: HashSet<Coordinate> = get_antenna_pairs(&map.antennas)
        .iter()
        .flat_map(|antennas| project_resonant_anti_nodes(antennas, &map))
        .filter(|coordinate| filter_off_map_coordinates(&map, coordinate))
        .collect();

    Some(anti_nodes.len().to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Frequency = char;
type Coordinate = (i32, i32);
type Antennas = HashMap<Frequency, Vec<Coordinate>>;

/*-----------------------------------------------------------------------------
  Parse Input File
-----------------------------------------------------------------------------*/

fn parse_input(input: &str) -> Map {
    let rows = input.lines().count() as i32;
    let columns = input.lines().next().unwrap().chars().count() as i32;

    let mut antennas = Antennas::new();
    for (row, line) in input.lines().enumerate() {
        for (column, frequency) in line.chars().enumerate() {
            if frequency != '.' {
                antennas
                    .entry(frequency)
                    .or_default()
                    .push((row as i32, column as i32));
            }
        }
    }

    Map {
        rows,
        columns,
        antennas,
    }
}

/*-----------------------------------------------------------------------------
  Map
-----------------------------------------------------------------------------*/

struct Map {
    rows: i32,
    columns: i32,
    antennas: Antennas,
}

/*-----------------------------------------------------------------------------
  Pipeline Functions
-----------------------------------------------------------------------------*/

fn get_antenna_pairs(antennas: &Antennas) -> Vec<[Coordinate; 2]> {
    let mut pairs: Vec<[Coordinate; 2]> = Vec::new();

    for coordinates in antennas.values() {
        for combination in coordinates.iter().cloned().tuple_combinations::<(_, _)>() {
            pairs.push([combination.0, combination.1]);
        }
    }

    pairs
}

fn project_anti_nodes(antennas: &[Coordinate; 2]) -> [Coordinate; 2] {
    let [a, b] = antennas;

    let difference = (b.0 - a.0, b.1 - a.1);

    let anti_node1 = (a.0 - difference.0, a.1 - difference.1);
    let anti_node2 = (b.0 + difference.0, b.1 + difference.1);

    [anti_node1, anti_node2]
}

fn project_resonant_anti_nodes(antennas: &[Coordinate; 2], map: &Map) -> Vec<Coordinate> {
    let [a, b] = antennas;

    let difference = (b.0 - a.0, b.1 - a.1);

    let mut node: i32 = 0;
    let mut resonant_anti_nodes: Vec<Coordinate> = Vec::new();
    loop {
        let a_anti_node = (a.0 - node * difference.0, a.1 - node * difference.1);
        let b_anti_node = (b.0 + node * difference.0, b.1 + node * difference.1);

        let filter_a_anti_node = filter_off_map_coordinates(map, &a_anti_node);
        let filter_b_anti_node = filter_off_map_coordinates(map, &b_anti_node);

        if filter_a_anti_node {
            resonant_anti_nodes.push(a_anti_node);
        }

        if filter_b_anti_node {
            resonant_anti_nodes.push(b_anti_node);
        }

        if !filter_a_anti_node && !filter_b_anti_node {
            break;
        }

        node += 1;
    }

    resonant_anti_nodes
}

fn filter_off_map_coordinates(map: &Map, coordinate: &Coordinate) -> bool {
    let (row, column) = coordinate;

    *row >= 0 && *row < map.rows && *column >= 0 && *column < map.columns
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 8: Resonant Collinearity")]
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
    use crate::get_answer;

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1(&get_input("../data/day8/example.txt")),
            get_answer("../data/day8/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day8/example.txt")),
            get_answer("../data/day8/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day8/input.txt")),
            get_answer("../data/day8/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day8/input.txt")),
            get_answer("../data/day8/input-part2-answer.txt")
        );
    }
}
