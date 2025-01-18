use crate::{get_input, GridDirection, GridIndex, GridMap};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter::once;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 16: Reindeer Maze
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let map = parse_input(input);
    let (best_score, _) = race(&map);

    Some(best_score.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let map = parse_input(input);
    let (_, best_paths_tile_count) = race(&map);

    Some(best_paths_tile_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Index = i16;
type TileCount = i16;
type Score = i32;
type PositionAndDirection = (GridIndex<Index>, GridDirection);

fn parse_input(input: &str) -> GridMap<Index, char> {
    input.into()
}

// Use Dijkstra's algorithm to find the shortest paths from the start to the goal
fn race(map: &GridMap<Index, char>) -> (Score, TileCount) {
    let start = map.find(|&c| c == 'S').unwrap();
    let goal = map.find(|&c| c == 'E').unwrap();

    let mut dist: HashMap<(GridIndex<Index>, GridDirection), Score> = HashMap::new();
    let mut prev: HashMap<PositionAndDirection, HashSet<PositionAndDirection>> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    let mut best_score: Score = Score::MAX;

    let initial_state = State {
        position: start,
        direction: GridDirection::Right,
        score: 0,
    };
    let initial_key = (initial_state.position, initial_state.direction);
    dist.insert(initial_key, initial_state.score);
    prev.insert(initial_key, HashSet::new());

    heap.push(initial_state);
    while let Some(current_state) = heap.pop() {
        let current_key = (current_state.position, current_state.direction);
        let current_tile_score = *dist.get(&current_key).unwrap_or(&Score::MAX);

        if current_state.position == goal && current_state.score < best_score {
            best_score = current_state.score;
            continue;
        }

        if current_state.score > current_tile_score || current_state.score >= best_score {
            continue;
        }

        for next_state in current_state.next_states(map).iter().filter_map(|s| *s) {
            let next_key = (next_state.position, next_state.direction);
            let next_tile_score = dist.get(&next_key).unwrap_or(&Score::MAX);

            match next_state.score.cmp(next_tile_score) {
                Ordering::Less => {
                    dist.insert(next_key, next_state.score);
                    prev.insert(next_key, once(current_key).collect());
                    heap.push(next_state);
                }
                Ordering::Equal => {
                    prev.entry(next_key).or_default().insert(current_key);
                    heap.push(next_state);
                }
                _ => {}
            }
        }
    }

    let mut tiles: HashSet<GridIndex<Index>> = HashSet::new();
    let mut stack = vec![
        (goal, GridDirection::Up),
        (goal, GridDirection::Down),
        (goal, GridDirection::Right),
        (goal, GridDirection::Left),
    ];
    while let Some(current) = stack.pop() {
        if let Some(prevs) = prev.get(&current) {
            tiles.insert(current.0);
            for prev in prevs {
                stack.push(*prev);
            }
        }
    }

    (best_score, tiles.len() as TileCount)
}

/*-----------------------------------------------------------------------------
  State
-----------------------------------------------------------------------------*/

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: GridIndex<Index>,
    direction: GridDirection,
    score: Score,
}

impl State {
    fn next_states(&self, map: &GridMap<Index, char>) -> [Option<Self>; 3] {
        [
            self.direction_is_clear(map, self.direction)
                .then_some(State {
                    position: map
                        .project_direction(self.position, self.direction)
                        .unwrap(),
                    direction: self.direction,
                    score: self.score + 1,
                }),
            self.direction_is_clear(map, self.direction.turn_right())
                .then_some(State {
                    position: self.position,
                    direction: self.direction.turn_right(),
                    score: self.score + 1000,
                }),
            self.direction_is_clear(map, self.direction.turn_left())
                .then_some(State {
                    position: self.position,
                    direction: self.direction.turn_left(),
                    score: self.score + 1000,
                }),
        ]
    }

    fn direction_is_clear(&self, map: &GridMap<Index, char>, direction: GridDirection) -> bool {
        let new_position = map.project_direction(self.position, direction).unwrap();
        map.get(new_position) != Some(&'#')
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to make the heap a min-heap
        other
            .score
            .cmp(&self.score)
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
#[command(long_about = "Day 16: Reindeer Maze")]
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
    fn test_example0_part1() {
        assert_eq!(
            part1(&get_input("../data/day16/example0.txt")),
            get_answer("../data/day16/example0-part1-answer.txt")
        );
    }

    #[test]
    fn test_example1_part1() {
        assert_eq!(
            part1(&get_input("../data/day16/example1.txt")),
            get_answer("../data/day16/example1-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day16/input.txt")),
            get_answer("../data/day16/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example0_part2() {
        assert_eq!(
            part2(&get_input("../data/day16/example0.txt")),
            get_answer("../data/day16/example0-part2-answer.txt")
        );
    }

    #[test]
    fn test_example1_part2() {
        assert_eq!(
            part2(&get_input("../data/day16/example1.txt")),
            get_answer("../data/day16/example1-part2-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day16/input.txt")),
            get_answer("../data/day16/input-part2-answer.txt")
        );
    }
}
