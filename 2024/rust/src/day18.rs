use crate::shared::map::{Direction4C, Map, MapIndex};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;
use std::path::Path;

/*-------------------------------------------------------------------------------------------------
  Day 18: RAM Run
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
fn example_part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let corrupted_memory_positions = parse_input_file(input);
    let mut map = Map::new(6 + 1, 6 + 1, '.');
    for position in corrupted_memory_positions.iter().take(12) {
        map.set(*position, '#');
    }

    let number_of_steps_to_exit = escape_route(&map);
    match number_of_steps_to_exit {
        Some(steps) => steps.to_string(),
        None => "No escape route found!".to_string(),
    }
}

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let corrupted_memory_positions = parse_input_file(input);
    let mut map = Map::new(70 + 1, 70 + 1, '.');
    for position in corrupted_memory_positions.iter().take(1024) {
        map.set(*position, '#');
    }

    let number_of_steps_to_exit = escape_route(&map);
    match number_of_steps_to_exit {
        Some(steps) => steps.to_string(),
        None => "No escape route found!".to_string(),
    }
}

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let corrupted_memory_positions = parse_input_file(input);
    let mut map = Map::new(70 + 1, 70 + 1, '.');

    for position in corrupted_memory_positions.iter().take(1024) {
        map.set(*position, '#');
    }

    let mut death_block: MapIndex = (0, 0);

    for position in corrupted_memory_positions.iter().skip(1024) {
        map.set(*position, '#');
        if escape_route(&map).is_none() {
            death_block = *position;
            break;
        };
    }

    format!("{x},{y}", x = death_block.1, y = death_block.0) // MapIndex is (row, column)
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Steps = u64;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Vec<MapIndex> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            (y, x) // MapIndex is (row, column)
        })
        .collect()
}

// Use Dijkstra's algorithm to find the shortest paths from the start to the goal
fn escape_route(map: &Map<char>) -> Option<Steps> {
    let start = (0, 0);
    let goal = (map.rows() - 1, map.columns() - 1);

    let mut dist: HashMap<MapIndex, Steps> = HashMap::new();
    let mut prev: HashMap<MapIndex, MapIndex> = HashMap::new();
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
    position: MapIndex,
    steps: Steps,
}

impl State {
    fn next_states(&self, map: &Map<char>) -> [Option<Self>; 4] {
        [
            Direction4C::North,
            Direction4C::South,
            Direction4C::East,
            Direction4C::West,
        ]
        .map(|direction| {
            let next_position = map.project_index_direction(self.position, direction)?;
            position_is_clear(map, next_position).then_some(State {
                position: next_position,
                steps: self.steps + 1,
            })
        })
    }
}

fn position_is_clear(map: &Map<char>, position: MapIndex) -> bool {
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
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            example_part1("../data/day18/example.txt"),
            solution("../data/day18/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day18/input.txt"),
            solution("../data/day18/input-part1-answer.txt")
        );
    }

    // #[test]
    // fn test_part2_solution() {
    //     assert_eq!(
    //         part2("../data/day18/input.txt"),
    //         solution("../data/day18/input-part2-answer.txt")
    //     );
    // }
}
