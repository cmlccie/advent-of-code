use crate::shared::inputs::get_input;
use crate::shared::map::{Map, MapIndex, Offset};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 15: Warehouse Woes
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let (mut warehouse, directions) = parse_input(input);

    let robot_starting_position = warehouse
        .find(|item| matches!(item, WarehouseItem::Robot))
        .unwrap();

    let mut robot = Robot::new(robot_starting_position);

    for direction in directions {
        robot.attempt_move(&mut warehouse, direction);
    }

    let gps_coordinate_sum = calculate_gps_coordinates_sum(&warehouse);

    Some(gps_coordinate_sum.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let (warehouse, directions) = parse_input(input);
    let mut warehouse = modify_warehouse(&warehouse);
    log::debug!("Starting Warehouse:\n{}", warehouse);

    let robot_starting_position = warehouse
        .find(|item| matches!(item, WarehouseItem::Robot))
        .unwrap();

    let mut robot = Robot::new(robot_starting_position);

    for direction in directions {
        robot.attempt_move(&mut warehouse, direction);
        log::debug!("Direction: {}\n{}", direction, warehouse);
    }

    let gps_coordinate_sum = calculate_gps_coordinates_sum(&warehouse);

    Some(gps_coordinate_sum.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Position = MapIndex;

fn parse_input(input: &str) -> (Map<WarehouseItem>, Vec<Direction>) {
    let blank_line_index = input.lines().position(|line| line.is_empty()).unwrap();

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

fn modify_warehouse(warehouse: &Map<WarehouseItem>) -> Map<WarehouseItem> {
    warehouse
        .contents()
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|item| match item {
                    WarehouseItem::Robot => [WarehouseItem::Robot, WarehouseItem::Empty],
                    WarehouseItem::Box => [WarehouseItem::BigBoxLeft, WarehouseItem::BigBoxRight],
                    item => [*item, *item],
                })
                .collect::<Vec<_>>()
        })
        .collect()
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

/*-----------------------------------------------------------------------------
  Warehouse Item
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseItem {
    Wall,
    Empty,
    Box,
    BigBoxLeft,
    BigBoxRight,
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

impl From<WarehouseItem> for char {
    fn from(item: WarehouseItem) -> Self {
        match item {
            WarehouseItem::Wall => '#',
            WarehouseItem::Empty => '.',
            WarehouseItem::Box => 'O',
            WarehouseItem::BigBoxLeft => '[',
            WarehouseItem::BigBoxRight => ']',
            WarehouseItem::Robot => '@',
        }
    }
}

/*-----------------------------------------------------------------------------
  Direction
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy)]
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

impl From<Direction> for char {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
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
        if item_at_position_can_move(warehouse, self.position, direction) {
            move_item(warehouse, self.position, direction);

            let offset = direction.offset();
            let next_position = warehouse
                .project_index_offset(self.position, offset)
                .unwrap();
            self.position = next_position;
        }
    }
}

/*-----------------------------------------------------------------------------
  Warehouse Functions
-----------------------------------------------------------------------------*/

// Treat the left-half of the big box as the left box as the controlling side

fn item_at_position_can_move(
    warehouse: &Map<WarehouseItem>,
    position: Position,
    direction: Direction,
) -> bool {
    let item = warehouse.get(position).unwrap().to_owned();

    if matches!(item, WarehouseItem::Wall) {
        return false;
    }

    if matches!(item, WarehouseItem::Empty) {
        return true;
    }

    next_positions(warehouse, position, direction)
        .iter()
        .all(|&next_position| item_at_position_can_move(warehouse, next_position, direction))
}

fn move_item(warehouse: &mut Map<WarehouseItem>, position: Position, direction: Direction) {
    let item = warehouse.get(position).unwrap().to_owned();

    if matches!(item, WarehouseItem::Empty) {
        return;
    }

    // Move items out of the way
    next_positions(warehouse, position, direction)
        .iter()
        .for_each(|&next_position| move_item(warehouse, next_position, direction));

    // Move the current item
    match item {
        WarehouseItem::BigBoxLeft | WarehouseItem::BigBoxRight => {
            move_big_box(warehouse, position, direction)
        }
        _ => move_single_item(warehouse, position, direction),
    }
}

fn move_single_item(warehouse: &mut Map<WarehouseItem>, position: Position, direction: Direction) {
    let next_position = warehouse
        .project_index_offset(position, direction.offset())
        .unwrap();

    map_move(warehouse, position, next_position);
}

fn move_big_box(warehouse: &mut Map<WarehouseItem>, position: Position, direction: Direction) {
    let (left_position, right_position) = big_box_positions(warehouse, position);
    match direction {
        Direction::Up | Direction::Down => {
            move_single_item(warehouse, left_position, direction);
            move_single_item(warehouse, right_position, direction);
        }
        Direction::Left => {
            move_single_item(warehouse, left_position, direction);
            move_single_item(warehouse, right_position, direction);
        }
        Direction::Right => {
            move_single_item(warehouse, right_position, direction);
            move_single_item(warehouse, left_position, direction);
        }
    }
}

fn next_positions(
    warehouse: &Map<WarehouseItem>,
    position: Position,
    direction: Direction,
) -> Vec<Position> {
    let item = warehouse.get(position).unwrap().to_owned();
    let offset = direction.offset();
    match item {
        WarehouseItem::Empty => vec![],
        WarehouseItem::Robot => vec![warehouse.project_index_offset(position, offset).unwrap()],
        WarehouseItem::Box => vec![warehouse.project_index_offset(position, offset).unwrap()],
        WarehouseItem::BigBoxLeft | WarehouseItem::BigBoxRight => {
            let (left_position, right_position) = big_box_positions(warehouse, position);
            match direction {
                Direction::Up | Direction::Down => vec![
                    warehouse
                        .project_index_offset(left_position, offset)
                        .unwrap(),
                    warehouse
                        .project_index_offset(right_position, offset)
                        .unwrap(),
                ],
                Direction::Left => vec![warehouse
                    .project_index_offset(left_position, offset)
                    .unwrap()],
                Direction::Right => vec![warehouse
                    .project_index_offset(right_position, offset)
                    .unwrap()],
            }
        }
        _ => panic!("We should only be moving movable items!"),
    }
}

fn big_box_positions(warehouse: &Map<WarehouseItem>, position: Position) -> (Position, Position) {
    let item_at_position = warehouse.get(position).unwrap().to_owned();

    match item_at_position {
        WarehouseItem::BigBoxLeft => {
            let left_position = position;
            let right_offset = Direction::Right.offset();
            let right_position = warehouse
                .project_index_offset(left_position, right_offset)
                .unwrap();
            (left_position, right_position)
        }
        WarehouseItem::BigBoxRight => {
            let right_position = position;
            let left_offset = Direction::Left.offset();
            let left_position = warehouse
                .project_index_offset(right_position, left_offset)
                .unwrap();
            (left_position, right_position)
        }
        _ => panic!("This function should only be called on a big box position!"),
    }
}

fn map_move(warehouse: &mut Map<WarehouseItem>, from: Position, to: Position) {
    assert_eq!(warehouse.get(to).unwrap(), &WarehouseItem::Empty);
    let item = warehouse.get(from).unwrap().to_owned();
    warehouse.set(from, WarehouseItem::Empty);
    warehouse.set(to, item);
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 15: Warehouse Woes")]
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
    use crate::shared::answers::get_answer;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(&get_input("../data/day15/example0.txt")),
            get_answer("../data/day15/example0-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day15/input.txt")),
            get_answer("../data/day15/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(&get_input("../data/day15/example0.txt")),
            get_answer("../data/day15/example0-part2-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day15/input.txt")),
            get_answer("../data/day15/input-part2-answer.txt")
        );
    }
}
