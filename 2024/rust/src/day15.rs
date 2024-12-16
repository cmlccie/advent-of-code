use crate::shared::map::{Map, MapIndex, Offset};
use std::fs::read_to_string;
use std::path::Path;

/*-------------------------------------------------------------------------------------------------
  Day 14: Restroom Redoubt
-------------------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> i64 {
    let (mut warehouse, directions) = parse_input_file(input);

    let robot_starting_position = warehouse
        .find(|item| matches!(item, WarehouseItem::Robot))
        .unwrap();

    let mut robot = Robot::new(robot_starting_position);

    for direction in directions {
        robot.attempt_move(&mut warehouse, direction);
    }

    calculate_gps_coordinates_sum(&warehouse) as i64
}

pub fn part2<P: AsRef<Path> + ?Sized>(_input: &P) -> i64 {
    unimplemented!()
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Position = MapIndex;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> (Map<WarehouseItem>, Vec<Direction>) {
    let input = read_to_string(input).unwrap();

    let blank_line_index = input.lines().position(|line| line.is_empty()).unwrap();
    println!("blank_line_index: {}", blank_line_index);

    let warehouse: Map<WarehouseItem> = input
        .lines()
        .take(blank_line_index)
        .map(|line| {
            line.chars()
                .map(|c| WarehouseItem::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let directions = input
        .lines()
        .skip(blank_line_index + 1)
        .flat_map(|line| line.chars())
        .map(|c| Direction::try_from(c).unwrap())
        .collect();

    (warehouse, directions)
}

fn calculate_gps_coordinates_sum(warehouse: &Map<WarehouseItem>) -> usize {
    warehouse
        .indices()
        .filter(|index| {
            matches!(
                *warehouse.get(*index).unwrap(),
                WarehouseItem::Box | WarehouseItem::BigBoxLeft
            )
        })
        .map(|(row, column)| row * 100 + column)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseItem {
    Wall,
    Empty,
    Box,
    Robot,
}

impl TryFrom<char> for WarehouseItem {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> Offset {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

/*-----------------------------------------------------------------------------
  Robot
-----------------------------------------------------------------------------*/

#[derive(Debug)]
struct Robot {
    position: Position,
}

impl Robot {
    fn new(position: Position) -> Self {
        Self { position }
    }

    fn attempt_move(&mut self, warehouse: &mut Map<WarehouseItem>, direction: Direction) {
        match attempt_move(warehouse, self.position, direction) {
            MoveResult::Success(new_position) => self.position = new_position,
            MoveResult::Blocked => (),
        }
    }
}

/*-----------------------------------------------------------------------------
  Warehouse Functions
-----------------------------------------------------------------------------*/

fn attempt_move(
    warehouse: &mut Map<WarehouseItem>,
    position: Position,
    direction: Direction,
) -> MoveResult {
    let offset = direction.offset();
    let new_position = warehouse.project_index_offset(position, offset).unwrap();
    let item_at_new_position = warehouse.get(new_position).unwrap().to_owned();

    match item_at_new_position {
        WarehouseItem::Wall => MoveResult::Blocked,
        WarehouseItem::Empty => {
            move_item(warehouse, position, new_position);
            MoveResult::Success(new_position)
        }
        _ => match attempt_move(warehouse, new_position, direction) {
            MoveResult::Success(_) => {
                move_item(warehouse, position, new_position);
                MoveResult::Success(new_position)
            }
            MoveResult::Blocked => MoveResult::Blocked,
        },
    }
}

enum MoveResult {
    Success(Position),
    Blocked,
}

fn move_item(warehouse: &mut Map<WarehouseItem>, from: Position, to: Position) {
    assert_eq!(warehouse.get(to).unwrap(), &WarehouseItem::Empty);
    let item = warehouse.get(from).unwrap().to_owned();
    println!("Moving {:?} from {:?} to {:?}", item, from, to);
    warehouse.set(from, WarehouseItem::Empty);
    warehouse.set(to, item);
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
            part1("../data/day15/example.txt"),
            solution("../data/day15/example-part1-answer.txt").unwrap()
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day15/input.txt"),
            solution("../data/day15/input-part1-answer.txt").unwrap()
        );
    }

    // #[test]
    // fn test_part2_solution() {
    //     assert_eq!(
    //         part2("../data/day15/input.txt"),
    //         solution("../data/day15/input-part2-answer.txt").unwrap()
    //     );
    // }
}
