use crate::shared::map::{Map, MapIndex, Offset};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

/*-------------------------------------------------------------------------------------------------
  Day 16: Reindeer Maze
-------------------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let map = parse_input_file(input);
    let (best_score, _) = race(&map);
    best_score.to_string()
}

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let map = parse_input_file(input);
    let (_, best_paths_tile_count) = race(&map);
    best_paths_tile_count.to_string()
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Score = u64;
type TileCount = u64;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Map<char> {
    read_to_string(input).unwrap().as_str().into()
}

fn race(map: &Map<char>) -> (Score, TileCount) {
    let start = map.find(|&c| c == 'S').unwrap();
    let finish = map.find(|&c| c == 'E').unwrap();
    let reindeer = Reindeer::new(start, Direction::East);

    let mut best_score: Score = Score::MAX;
    let mut best_score_to_reach_tile: HashMap<MapIndex, Score> = HashMap::new();

    let mut best_path_tiles: HashSet<MapIndex> = HashSet::new();

    let mut stack = vec![reindeer];
    while let Some(reindeer) = stack.pop() {
        // Prune reindeer that can't beat the best score
        if reindeer.score > best_score {
            continue;
        }

        // Prune reindeer that have looped into their own path
        if reindeer.visited.contains(&reindeer.position) {
            continue;
        };

        // Prune reindeer the will have a higher score leaving the current tile
        if let Some(&score) = best_score_to_reach_tile.get(&reindeer.position) {
            if reindeer.score > (score + 1000) {
                continue;
            }
        };
        best_score_to_reach_tile.insert(reindeer.position, reindeer.score);

        if reindeer.finished(map) {
            match reindeer.score.cmp(&best_score) {
                std::cmp::Ordering::Less => {
                    best_score = reindeer.score;
                    best_path_tiles = reindeer.visited;
                }
                std::cmp::Ordering::Equal => {
                    best_path_tiles = best_path_tiles.union(&reindeer.visited).copied().collect();
                }
                _ => {}
            }
            continue;
        };

        let actions = reindeer.available_actions(map);

        // Prune reindeer that can't move forward or turn left or right
        if actions.iter().all(Option::is_none) {
            continue;
        };

        actions
            .iter()
            .filter_map(|action| *action)
            .for_each(|action| {
                stack.push(reindeer.fork(action, map));
            });
    }

    best_path_tiles.insert(finish);

    (best_score, best_path_tiles.len() as TileCount)
}

/*-----------------------------------------------------------------------------
  Reindeer
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone)]
struct Reindeer {
    position: MapIndex,
    direction: Direction,
    visited: HashSet<MapIndex>,
    score: u64,
}

impl Reindeer {
    fn new(position: MapIndex, direction: Direction) -> Self {
        Self {
            position,
            direction,
            visited: HashSet::new(),
            score: 0,
        }
    }

    fn finished(&self, map: &Map<char>) -> bool {
        map.get(self.position) == Some(&'E')
    }

    fn fork(&self, action: ReindeerAction, map: &Map<char>) -> Self {
        let mut reindeer = self.clone();
        reindeer.visited.insert(reindeer.position);

        match action {
            ReindeerAction::MoveForward => reindeer.move_forward(map),
            ReindeerAction::Turn(direction) => {
                reindeer.turn(direction);
                reindeer.move_forward(map)
            }
        };

        reindeer
    }

    fn available_actions(&self, map: &Map<char>) -> [Option<ReindeerAction>; 3] {
        let forward = self.direction;
        let left = self.direction.turn_counterclockwise();
        let right = self.direction.turn_clockwise();
        [
            self.direction_is_clear(map, left)
                .then_some(ReindeerAction::Turn(left)),
            self.direction_is_clear(map, forward)
                .then_some(ReindeerAction::MoveForward),
            self.direction_is_clear(map, right)
                .then_some(ReindeerAction::Turn(right)),
        ]
    }

    fn direction_is_clear(&self, map: &Map<char>, direction: Direction) -> bool {
        let offset = direction.offset();
        let new_position = map.project_index_offset(self.position, offset).unwrap();
        map.get(new_position) != Some(&'#')
    }

    fn move_forward(&mut self, map: &Map<char>) {
        let offset = self.direction.offset();
        self.position = map.project_index_offset(self.position, offset).unwrap();
        self.score += 1;
    }

    fn turn(&mut self, direction: Direction) {
        self.direction = direction;
        self.score += 1000;
    }
}

#[derive(Debug, Clone, Copy)]
enum ReindeerAction {
    MoveForward,
    Turn(Direction),
}

/*-----------------------------------------------------------------------------
  Direction
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
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

    fn turn_counterclockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
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
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
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
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day16/input.txt"),
            solution("../data/day16/input-part2-answer.txt")
        );
    }
}
