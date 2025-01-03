use crate::shared::map::{Direction4C, Map, MapIndex};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;
use std::iter::once;
use std::path::{Path, PathBuf};

/*-------------------------------------------------------------------------------------------------
  Day 16: Reindeer Maze
-------------------------------------------------------------------------------------------------*/

fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let map = parse_input_file(input);
    let (best_score, _) = race(&map);

    Some(best_score.to_string())
}

fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let map = parse_input_file(input);
    let (_, best_paths_tile_count) = race(&map);

    Some(best_paths_tile_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Score = u64;
type TileCount = u64;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Map<char> {
    read_to_string(input).unwrap().as_str().into()
}

// Use Dijkstra's algorithm to find the shortest paths from the start to the goal
fn race(map: &Map<char>) -> (Score, TileCount) {
    let start = map.find(|&c| c == 'S').unwrap();
    let goal = map.find(|&c| c == 'E').unwrap();

    let mut dist: HashMap<(MapIndex, Direction4C), Score> = HashMap::new();
    let mut prev: HashMap<(MapIndex, Direction4C), HashSet<(MapIndex, Direction4C)>> =
        HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    let mut best_score: Score = Score::MAX;

    let initial_state = State {
        position: start,
        direction: Direction4C::East,
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

    let mut tiles: HashSet<MapIndex> = HashSet::new();
    let mut stack = vec![
        (goal, Direction4C::North),
        (goal, Direction4C::South),
        (goal, Direction4C::East),
        (goal, Direction4C::West),
    ];
    while let Some(current) = stack.pop() {
        if let Some(prevs) = prev.get(&current) {
            tiles.insert(current.0);
            for prev in prevs {
                stack.push(*prev);
            }
        }
    }
    log::debug!("Best Paths:\n{}", map.display_with_overlay('O', &tiles));

    (best_score, tiles.len() as TileCount)
}

/*-----------------------------------------------------------------------------
  State
-----------------------------------------------------------------------------*/

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: MapIndex,
    direction: Direction4C,
    score: Score,
}

impl State {
    fn next_states(&self, map: &Map<char>) -> [Option<Self>; 3] {
        [
            self.direction_is_clear(map, self.direction)
                .then_some(State {
                    position: map
                        .project_index_direction(self.position, self.direction)
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

    fn direction_is_clear(&self, map: &Map<char>, direction: Direction4C) -> bool {
        let new_position = map
            .project_index_direction(self.position, direction)
            .unwrap();
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
    fn test_example0_part1() {
        assert_eq!(
            part1("../data/day16/example0.txt"),
            solution("../data/day16/example0-part1-answer.txt")
        );
    }

    #[test]
    fn test_example1_part1() {
        assert_eq!(
            part1("../data/day16/example1.txt"),
            solution("../data/day16/example1-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day16/input.txt"),
            solution("../data/day16/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example0_part2() {
        assert_eq!(
            part2("../data/day16/example0.txt"),
            solution("../data/day16/example0-part2-answer.txt")
        );
    }

    #[test]
    fn test_example1_part2() {
        assert_eq!(
            part2("../data/day16/example1.txt"),
            solution("../data/day16/example1-part2-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day16/input.txt"),
            solution("../data/day16/input-part2-answer.txt")
        );
    }
}
