use crate::{get_input, GridDirection, GridIndex, GridMap};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 18: RAM Run
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let corrupted_memory_positions = parse_input(input);
    let mut map = GridMap::new(70 + 1, 70 + 1, '.');
    for position in corrupted_memory_positions.iter().take(1024) {
        map.set(*position, '#').unwrap();
    }

    let number_of_steps_to_exit = escape_route(&map);

    number_of_steps_to_exit.map(|steps| steps.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let corrupted_memory_positions = parse_input(input);
    let mut map = GridMap::new(70 + 1, 70 + 1, '.');

    for position in corrupted_memory_positions.iter().take(1024) {
        map.set(*position, '#').unwrap();
    }

    let mut death_block: GridIndex<Index> = GridIndex::new(0, 0);

    for position in corrupted_memory_positions.iter().skip(1024) {
        map.set(*position, '#').unwrap();
        if escape_route(&map).is_none() {
            death_block = *position;
            break;
        };
    }

    // GridIndex<Index> is (row, column)
    Some(format!(
        "{x},{y}",
        x = death_block.column,
        y = death_block.row
    ))
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Index = i8;
type Steps = u16;

fn parse_input(input: &str) -> Vec<GridIndex<Index>> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            (y, x).into() // GridIndex<Index> is (row, column)
        })
        .collect()
}

// Use Dijkstra's algorithm to find the shortest paths from the start to the goal
fn escape_route(map: &GridMap<Index, char>) -> Option<Steps> {
    let start = (0, 0).into();
    let goal = (map.rows() - 1, map.columns() - 1).into();

    let mut dist: HashMap<GridIndex<Index>, Steps> = HashMap::new();
    let mut prev: HashMap<GridIndex<Index>, GridIndex<Index>> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    let initial = State {
        position: start,
        steps: 0,
    };
    dist.insert(initial.position, initial.steps);
    heap.push(initial);
    while let Some(current) = heap.pop() {
        if current.position == goal {
            break;
        }

        let current_tile_best_steps = *dist.get(&current.position).unwrap_or(&Steps::MAX);
        if current.steps > current_tile_best_steps {
            continue;
        }

        for next in current.next_states(map).iter().filter_map(|s| *s) {
            let next_tile_best_steps = *dist.get(&next.position).unwrap_or(&Steps::MAX);
            if next.steps < next_tile_best_steps {
                dist.insert(next.position, next.steps);
                prev.insert(next.position, current.position);
                heap.push(next);
            }
        }
    }

    dist.get(&goal).copied()
}

/*-----------------------------------------------------------------------------
  State
-----------------------------------------------------------------------------*/

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: GridIndex<Index>,
    steps: Steps,
}

impl State {
    fn next_states(&self, map: &GridMap<Index, char>) -> [Option<Self>; 4] {
        [
            GridDirection::Up,
            GridDirection::Down,
            GridDirection::Right,
            GridDirection::Left,
        ]
        .map(|direction| {
            let next_position = map.project_direction(self.position, direction)?;
            position_is_clear(map, next_position).then_some(State {
                position: next_position,
                steps: self.steps + 1,
            })
        })
    }
}

fn position_is_clear(map: &GridMap<Index, char>, position: GridIndex<Index>) -> bool {
    map.get(position) != Some(&'#')
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to make the heap a min-heap
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 18: RAM Run")]
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
fn example_part1(input: &str) -> Option<String> {
    let corrupted_memory_positions = parse_input(input);
    let mut map = GridMap::new(6 + 1, 6 + 1, '.');
    for position in corrupted_memory_positions.iter().take(12) {
        map.set(*position, '#').unwrap();
    }

    let number_of_steps_to_exit = escape_route(&map);

    number_of_steps_to_exit.map(|steps| steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_answer;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            example_part1(&get_input("../data/day18/example.txt")),
            get_answer("../data/day18/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day18/input.txt")),
            get_answer("../data/day18/input-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day18/input.txt")),
            get_answer("../data/day18/input-part2-answer.txt")
        );
    }
}
