/*-------------------------------------------------------------------------------------------------
  Day 6: Guard Gallivant
-------------------------------------------------------------------------------------------------*/

use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

/*--------------------------------------------------------------------------------------
  Part 1
--------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> i64 {
    let map = parse_input_file(input);

    let start_position = find_guard_start_position(&map);
    let mut guard = Guard::new(start_position, Direction::North);

    while guard.next(&map) != Action::Exit {}

    let visited: HashSet<(usize, usize)> =
        guard.route.iter().map(|(position, _)| *position).collect();
    visited.len() as i64
}

/*--------------------------------------------------------------------------------------
  Part 2
--------------------------------------------------------------------------------------*/

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> i64 {
    let mut map = parse_input_file(input);

    let start_position = find_guard_start_position(&map);
    let mut guard = Guard::new(start_position, Direction::North);

    while guard.next(&map) != Action::Exit {}
    let mut possible_positions = guard.route;
    possible_positions.remove(&(start_position, Direction::North));

    let mut loop_obstruction_positions = HashSet::new();

    for (position, _) in possible_positions {
        let saved_tile = map.get(position.0, position.1).unwrap();
        map.map[position.0][position.1] = '#';

        let mut sim_guard = Guard::new(start_position, Direction::North);
        loop {
            match sim_guard.next(&map) {
                Action::Loop => {
                    loop_obstruction_positions.insert(position);
                    break;
                }
                Action::Exit => break,
                _ => {}
            }
        }

        map.map[position.0][position.1] = saved_tile;
    }

    loop_obstruction_positions.len() as i64
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
  Parse Input File
-----------------------------------------------------------------------------*/

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Map {
    let map: Vec<Vec<char>> = read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Map::new(map)
}

/*-----------------------------------------------------------------------------
  Map
-----------------------------------------------------------------------------*/

struct Map {
    map: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Self {
        let rows = map.len();
        let columns = map[0].len();
        Self { map, rows, columns }
    }

    fn get(&self, row: usize, column: usize) -> Option<char> {
        if row < self.rows && column < self.columns {
            Some(self.map[row][column])
        } else {
            None
        }
    }

    fn project(
        &self,
        coordinate: &(usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        let offset = direction.offset();

        let new_coordinate = (
            coordinate.0 as i32 + offset.0,
            coordinate.1 as i32 + offset.1,
        );

        let new_coordinate = if new_coordinate.0 >= 0 && new_coordinate.1 >= 0 {
            (new_coordinate.0 as usize, new_coordinate.1 as usize)
        } else {
            return None;
        };

        if new_coordinate.0 < self.rows && new_coordinate.1 < self.columns {
            Some(new_coordinate)
        } else {
            None
        }
    }
}

/*-----------------------------------------------------------------------------
  Guard
-----------------------------------------------------------------------------*/

struct Guard {
    position: (usize, usize),
    direction: Direction,

    route: HashSet<((usize, usize), Direction)>,
}

impl Guard {
    fn new(position: (usize, usize), direction: Direction) -> Self {
        let mut route = HashSet::new();
        route.insert((position, direction));

        Self {
            position,
            direction,
            route,
        }
    }

    fn next(&mut self, map: &Map) -> Action {
        let space_in_front = map.project(&self.position, &self.direction);
        if space_in_front.is_none() {
            return Action::Exit;
        }

        let space_in_front = space_in_front.unwrap();
        let contents_of_space = map.get(space_in_front.0, space_in_front.1).unwrap();
        if contents_of_space == '#' {
            self.turn()
        } else {
            self.move_to(space_in_front)
        }
    }

    fn turn(&mut self) -> Action {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        if self.in_loop() {
            return Action::Loop;
        }

        self.route.insert((self.position, self.direction));
        Action::Turn
    }

    fn move_to(&mut self, new_position: (usize, usize)) -> Action {
        self.position = new_position;

        if self.in_loop() {
            return Action::Loop;
        }

        self.route.insert((self.position, self.direction));
        Action::Move
    }

    fn in_loop(&self) -> bool {
        self.route.contains(&(self.position, self.direction))
    }
}

fn find_guard_start_position(map: &Map) -> (usize, usize) {
    for row in 0..map.rows {
        for column in 0..map.columns {
            if map.get(row, column).unwrap() == '^' {
                return (row, column);
            }
        }
    }

    panic!("Guard not found");
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
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

/*-----------------------------------------------------------------------------
  Actions
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Turn,
    Move,
    Exit,
    Loop,
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
            part1("../data/day6/example.txt"),
            solution("../data/day6/example-part1-answer.txt").unwrap()
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2("../data/day6/example.txt"),
            solution("../data/day6/example-part2-answer.txt").unwrap()
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day6/input.txt"),
            solution("../data/day6/input-part1-answer.txt").unwrap()
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day6/input.txt"),
            solution("../data/day6/input-part2-answer.txt").unwrap()
        );
    }
}
