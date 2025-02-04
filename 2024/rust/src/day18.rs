use crate::{get_input, GridDirection, GridIndex, GridMap};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 18: RAM Run
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str, grid_size: Index, initial_corrupt_blocks: usize) -> Option<String> {
    let corrupted_memory_positions = parse_input(input);
    let mut map = GridMap::new(grid_size + 1, grid_size + 1, '.');
    for position in corrupted_memory_positions
        .iter()
        .take(initial_corrupt_blocks)
    {
        map.set(*position, '#').unwrap();
    }

    let (steps_to_exit, _) = escape_route(&map).unwrap();

    Some(steps_to_exit.to_string())
}

pub fn part2(input: &str, grid_size: Index, initial_corrupt_blocks: usize) -> Option<String> {
    let corrupted_memory_positions = parse_input(input);
    let mut map = GridMap::new(grid_size + 1, grid_size + 1, '.');

    for position in corrupted_memory_positions
        .iter()
        .take(initial_corrupt_blocks)
    {
        map.set(*position, '#').unwrap();
    }

    let mut death_block: Position = Position::new(0, 0);
    let mut current_escape_route = escape_route(&map).map(|(_, route)| route);

    for position in corrupted_memory_positions
        .iter()
        .skip(initial_corrupt_blocks)
    {
        map.set(*position, '#').unwrap();

        if current_escape_route
            .as_ref()
            .map(|route| route.contains(position))
            .unwrap_or(false)
        {
            // Previous route blocked; try to find a new escape route
            current_escape_route = escape_route(&map).map(|(_, route)| route);
        }

        if current_escape_route.is_none() {
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

type Index = i16;
type Position = GridIndex<Index>;
type Steps = u16;
type Route = HashSet<Position>;

fn parse_input(input: &str) -> Vec<Position> {
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
fn escape_route(map: &GridMap<Index, char>) -> Option<(Steps, Route)> {
    let start: Position = (0, 0).into();
    let goal: Position = (map.rows() - 1, map.columns() - 1).into();

    let mut dist: GridMap<Index, Steps> = GridMap::new(map.rows(), map.columns(), Steps::MAX);
    let mut prev: GridMap<Index, Option<Position>> = GridMap::new(map.rows(), map.columns(), None);
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    let initial = State::new(start, 0, goal);
    dist.set(initial.position, initial.steps).unwrap();
    heap.push(initial);
    while let Some(current) = heap.pop() {
        if current.position == goal {
            break;
        }

        let current_tile_best_steps = *dist.get(current.position).unwrap();
        if current.steps > current_tile_best_steps {
            continue;
        }

        for next in current.next_states(map) {
            let next_tile_best_steps = *dist.get(next.position).unwrap_or(&Steps::MAX);
            if next.steps < next_tile_best_steps {
                dist.set(next.position, next.steps).unwrap();
                prev.set(next.position, Some(current.position)).unwrap();
                heap.push(next);
            }
        }
    }

    if let Some(steps) = dist.get(goal).copied() {
        // Reconstruct the route
        let route = {
            let mut route = Route::new();
            let mut current = goal;
            while let Some(previous) = prev.get(current).unwrap() {
                route.insert(current);
                current = *previous;
            }
            route
        };

        Some((steps, route))
    } else {
        None
    }
}

/*-----------------------------------------------------------------------------
  State
-----------------------------------------------------------------------------*/

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    steps: Steps,
    goal: Position,
    distance_to_goal: Index,
}

impl State {
    fn new(position: Position, steps: Steps, goal: Position) -> Self {
        Self {
            position,
            steps,
            goal,
            distance_to_goal: position.manhattan_distance(goal),
        }
    }

    fn next_states<'g>(
        &self,
        map: &'g GridMap<Index, char>,
    ) -> impl Iterator<Item = State> + use<'g, '_> {
        [
            GridDirection::Down,
            GridDirection::Right,
            GridDirection::Up,
            GridDirection::Left,
        ]
        .iter()
        .filter_map(|direction| {
            let (next_position, next_tile) =
                map.get_direction_index_and_value(self.position, *direction);
            next_position.and_then(|next_position| {
                (next_tile != Some(&'#')).then_some(State::new(
                    next_position,
                    self.steps + 1,
                    self.goal,
                ))
            })
        })
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the comparison to make the heap a min-heap
        (other.steps, other.distance_to_goal).cmp(&(self.steps, self.distance_to_goal))
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
    Part1 {
        input: PathBuf,

        #[clap(short, long, default_value = "70")]
        grid_size: Index,

        #[clap(short, long, default_value = "1024")]
        initial_corrupt_blocks: usize,
    },
    Part2 {
        input: PathBuf,

        #[clap(short, long, default_value = "70")]
        grid_size: Index,

        #[clap(short, long, default_value = "1024")]
        initial_corrupt_blocks: usize,
    },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 {
            input,
            grid_size,
            initial_corrupt_blocks,
        } => part1(&get_input(&input), grid_size, initial_corrupt_blocks),
        Args::Part2 {
            input,
            grid_size,
            initial_corrupt_blocks,
        } => part2(&get_input(&input), grid_size, initial_corrupt_blocks),
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
    fn test_example_part1() {
        assert_eq!(
            part1(&get_input("../data/day18/example.txt"), 6, 12),
            get_answer("../data/day18/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day18/input.txt"), 70, 1024),
            get_answer("../data/day18/input-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day18/input.txt"), 70, 1024),
            get_answer("../data/day18/input-part2-answer.txt")
        );
    }
}
