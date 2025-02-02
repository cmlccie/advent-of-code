use crate::get_input;
use std::collections::HashSet;
use std::hash::Hash;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 10: Hoof It
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let map = parse_input(input);
    let (_, peak_count) = map_trails(&map);

    Some(peak_count.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let map = parse_input(input);
    let (trail_count, _) = map_trails(&map);

    Some(trail_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

fn parse_input(input: &str) -> Map {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    Map::new(map)
}

fn map_trails(map: &Map) -> (TrailCount, PeakCount) {
    map.get_trailheads()
        .iter()
        .map(|trailhead| {
            let hiker = Hiker::new(*trailhead);
            let (trail_count, peaks) = hike_trails(map, hiker);
            let peak_count = peaks.len();
            (trail_count, peak_count)
        })
        .reduce(|acc, value| (acc.0 + value.0, acc.1 + value.1))
        .unwrap()
}

fn hike_trails(map: &Map, hiker: Hiker) -> (TrailCount, HashSet<Peak>) {
    // Base cases
    if map.get(hiker.position).unwrap() == 9 {
        let mut peaks = HashSet::new();
        peaks.insert(hiker.position);
        return (1, peaks);
    };

    let available_directions = hiker.available_directions(map);

    if available_directions.is_empty() {
        return (0, HashSet::new());
    }

    // Recursive case
    available_directions
        .iter()
        .fold((0, HashSet::new()), |acc, direction| {
            let mut hiker = hiker.clone();
            hiker.r#move(map, direction);
            let (trail_count, peaks) = hike_trails(map, hiker);

            (acc.0 + trail_count, acc.1.union(&peaks).cloned().collect())
        })
}

/*-----------------------------------------------------------------------------
  Map
-----------------------------------------------------------------------------*/

type Height = u32;
type Location = (usize, usize);
type Offset = (i64, i64);
type TrailCount = usize;
type Peak = Location;
type PeakCount = usize;

struct Map {
    map: Vec<Vec<Height>>,
    rows: usize,
    columns: usize,
}

impl Map {
    fn new(map: Vec<Vec<Height>>) -> Self {
        let rows = map.len();
        let columns = map[0].len();
        Self { map, rows, columns }
    }

    fn get_trailheads(&self) -> Vec<Location> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(row, columns)| {
                columns
                    .iter()
                    .enumerate()
                    .filter_map(|(column, &height)| {
                        if height == 0 {
                            Some((row, column))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Location>>()
            })
            .collect()
    }

    fn get(&self, location: Location) -> Option<Height> {
        let (row, column) = location;
        if row < self.rows && column < self.columns {
            Some(self.map[row][column])
        } else {
            None
        }
    }

    fn project_location(&self, location: &Location, direction: &Direction) -> Option<Location> {
        let (row, column) = location;
        let (row_shift, column_shift) = direction.offset();

        let (new_row, new_column) = (*row as i64 + row_shift, *column as i64 + column_shift);

        let new_location = if new_row >= 0 && new_column >= 0 {
            (new_row as usize, new_column as usize)
        } else {
            return None;
        };

        if new_location.0 < self.rows && new_location.1 < self.columns {
            Some(new_location)
        } else {
            None
        }
    }

    fn get_projected_value(&self, location: &Location, direction: &Direction) -> Option<Height> {
        self.project_location(location, direction)
            .and_then(|new_location| self.get(new_location))
    }
}

/*-----------------------------------------------------------------------------
  Hiker
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone)]
struct Hiker {
    position: Location,
}

impl Hiker {
    fn new(position: (usize, usize)) -> Self {
        let mut route = HashSet::new();
        route.insert(position);

        Self { position }
    }

    fn available_directions(&self, map: &Map) -> Vec<Direction> {
        let current_height = map.get(self.position).unwrap();
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        .filter_map(|direction| {
            let next_height = map.get_projected_value(&self.position, direction)?;
            if next_height == current_height + 1 {
                Some(*direction)
            } else {
                None
            }
        })
        .collect()
    }

    fn r#move(&mut self, map: &Map, direction: &Direction) {
        self.position = map.project_location(&self.position, direction).unwrap();
    }
}

/*-----------------------------------------------------------------------------
  Direction
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn offset(&self) -> Offset {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 10: Hoof It")]
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
            part1(&get_input("../data/day10/example.txt")),
            get_answer("../data/day10/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day10/example.txt")),
            get_answer("../data/day10/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day10/input.txt")),
            get_answer("../data/day10/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day10/input.txt")),
            get_answer("../data/day10/input-part2-answer.txt")
        );
    }
}
