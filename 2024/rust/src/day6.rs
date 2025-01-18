use crate::{get_input, GridDirection, GridIndex, GridMap};
use std::collections::HashSet;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 6: Guard Gallivant
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let (map, mut guard) = parse_input(input);

    while guard.next(&map) != Action::Exit {}

    let visited_positions: HashSet<Position> =
        guard.route.iter().map(|(position, _)| *position).collect();

    Some(visited_positions.len().to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let (mut map, mut guard) = parse_input(input);

    let mut checked_positions = HashSet::new();
    let mut loop_obstruction_positions = HashSet::new();

    loop {
        if let Some(next_position) = map.project_direction(guard.position, guard.direction) {
            let contents = map.get(next_position).copied();
            if matches!(contents, Some('.')) && !checked_positions.contains(&next_position) {
                let obstacle_position = next_position;
                let original_tile = contents.unwrap();

                map.set(obstacle_position, '#').unwrap();

                let mut virtual_guard = guard.clone();
                loop {
                    match virtual_guard.next(&map) {
                        Action::Loop => {
                            loop_obstruction_positions.insert(obstacle_position);
                            break;
                        }
                        Action::Exit => break,
                        _ => {}
                    }
                }

                map.set(obstacle_position, original_tile).unwrap();
                checked_positions.insert(obstacle_position);
            }
        }

        match guard.next(&map) {
            Action::Loop => break,
            Action::Exit => break,
            _ => continue,
        }
    }

    Some(loop_obstruction_positions.len().to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Index = i16;
type Map = GridMap<Index, char>;
type Position = GridIndex<Index>;

fn parse_input(input: &str) -> (GridMap<Index, char>, Guard) {
    let map: GridMap<Index, char> = input.into();
    let start_position = map.find(|c| c == &'^').unwrap();
    let guard = Guard::new(start_position, GridDirection::Up);
    (map, guard)
}

/*-----------------------------------------------------------------------------
  Guard
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone)]
struct Guard {
    position: Position,
    direction: GridDirection,

    route: HashSet<(GridIndex<Index>, GridDirection)>,
}

impl Guard {
    fn new(position: Position, direction: GridDirection) -> Self {
        let mut route = HashSet::new();
        route.insert((position, direction));

        Self {
            position,
            direction,
            route,
        }
    }

    fn next(&mut self, map: &Map) -> Action {
        let space_in_front = map.project_direction(self.position, self.direction);
        match space_in_front.and_then(|position| map.get(position)) {
            Some('#') => self.turn_right(),
            Some(_) => self.move_to(space_in_front.unwrap()),
            None => Action::Exit,
        }
    }

    fn turn_right(&mut self) -> Action {
        self.direction = self.direction.turn_right();

        if self.in_loop() {
            return Action::Loop;
        }

        self.route.insert((self.position, self.direction));
        Action::Turn
    }

    fn move_to(&mut self, new_position: Position) -> Action {
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
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 6: Guard Gallivant")]
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
            part1(&get_input("../data/day6/example.txt")),
            get_answer("../data/day6/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day6/example.txt")),
            get_answer("../data/day6/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day6/input.txt")),
            get_answer("../data/day6/input-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day6/input.txt")),
            get_answer("../data/day6/input-part2-answer.txt")
        );
    }
}
