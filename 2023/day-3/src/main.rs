use regex::Regex;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{collections::HashSet, fs::File};

const NOT_SYMBOLS: &str = ".0123456789";
const SYMBOLS: &str = "%-+*#/@=$&";

/*-------------------------------------------------------------------------------------------------
  Main Function
-------------------------------------------------------------------------------------------------*/

fn main() {
    let input = PathBuf::from("input.txt");
    part1(input);
}

/*-------------------------------------------------------------------------------------------------
Part 1
-------------------------------------------------------------------------------------------------*/

fn part1(input: PathBuf) -> i32 {
    let engine_schematic = read_input(&input);

    let symbols = identify_symbols(&engine_schematic);
    println!("Symbols: {:?}", symbols);

    let part_numbers = get_part_numbers(&engine_schematic);
    let sum = part_numbers.iter().sum();

    println!("Part 1 Answer: {}", sum);

    sum
}

/*-------------------------------------------------------------------------------------------------
  Core Functions
-------------------------------------------------------------------------------------------------*/

fn get_part_numbers(engine_schematic: &Vec<String>) -> Vec<i32> {
    let bounds = ArrayBounds::new(&engine_schematic);
    let number_regex = Regex::new(r"\d+").unwrap();

    let part_numbers: Vec<i32> = engine_schematic
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            number_regex.find_iter(line).flat_map(move |regex_match| {
                let adjacent_spaces =
                    box_coordinates(row, regex_match.start(), regex_match.end(), bounds);
                if adjacent_spaces.iter().any(|space| {
                    SYMBOLS.contains(
                        engine_schematic[space.row]
                            .chars()
                            .nth(space.column)
                            .unwrap(),
                    )
                }) {
                    Some(regex_match.as_str().parse::<i32>().unwrap())
                } else {
                    None
                }
            })
        })
        .collect();

    part_numbers
}

/*-------------------------------------------------------------------------------------------------
  Helper Functions
-------------------------------------------------------------------------------------------------*/

fn read_input(path: &PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn identify_symbols(engine_schematic: &Vec<String>) -> String {
    let not_symbols: HashSet<char> = NOT_SYMBOLS.chars().collect();
    let symbol_set: HashSet<char> = engine_schematic
        .iter()
        .flat_map(|s| s.chars())
        .filter(|c| !not_symbols.contains(c))
        .collect();
    symbol_set.into_iter().collect()
}

/*--------------------------------------------------------------------------------------
  Helper Data Structures
--------------------------------------------------------------------------------------*/

#[derive(Debug, Copy, Clone)]
struct ArrayCoordinates {
    row: usize,
    column: usize,
}

#[derive(Debug, Copy, Clone)]
struct ArrayBounds {
    rows: usize,
    columns: usize,
}

impl ArrayBounds {
    fn new(array: &Vec<String>) -> ArrayBounds {
        let rows = array.len();
        let columns = array[0].len();
        ArrayBounds { rows, columns }
    }

    fn in_bounds(&self, row: usize, column: usize) -> bool {
        row < self.rows && column < self.columns
    }
}

/*--------------------------------------------------------------------------------------
  Box Coordinates
--------------------------------------------------------------------------------------*/

fn box_coordinates(
    row: usize,
    column_start: usize,
    column_end: usize,
    bounds: ArrayBounds,
) -> Vec<ArrayCoordinates> {
    (row.saturating_sub(1)..=row + 1)
        .into_iter()
        .flat_map(move |row| {
            (column_start.saturating_sub(1)..column_end + 1)
                .into_iter()
                .flat_map(move |column| {
                    if bounds.in_bounds(row, column) {
                        Some(ArrayCoordinates { row, column })
                    } else {
                        None
                    }
                })
        })
        .collect()
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    /*----------------------------------------------------------------------------------
      Unit Tests
    --------------------------------------------------------------------------------------*/

    #[test]
    fn test_array_bounds_new() {
        let array = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
        ];
        let bounds = ArrayBounds::new(&array);
        assert_eq!(bounds.rows, 3);
        assert_eq!(bounds.columns, 3);
    }

    #[test]
    fn test_array_bounds_in_bounds() {
        let array = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
        ];
        let bounds = ArrayBounds::new(&array);
        assert!(bounds.in_bounds(0, 0));
        assert!(bounds.in_bounds(2, 2));
        assert!(!bounds.in_bounds(3, 0));
        assert!(!bounds.in_bounds(0, 3));
    }

    #[test]
    fn test_box_coordinates() {
        // Test a full box around a single character.
        let bounds = ArrayBounds {
            rows: 3,
            columns: 3,
        };
        let coordinates = box_coordinates(1, 1, 2, bounds);
        assert_eq!(coordinates.len(), 9);
        assert_eq!(coordinates[0].row, 0);
        assert_eq!(coordinates[0].column, 0);
        assert_eq!(coordinates[8].row, 2);
        assert_eq!(coordinates[8].column, 2);

        // Test a box that is partially out of bounds.
        let bounds = ArrayBounds {
            rows: 2,
            columns: 2,
        };
        let coordinates = box_coordinates(0, 0, 1, bounds);
        assert_eq!(coordinates.len(), 4);
        assert_eq!(coordinates[0].row, 0);
        assert_eq!(coordinates[0].column, 0);
        assert_eq!(coordinates[3].row, 1);
        assert_eq!(coordinates[3].column, 1);
    }

    /*----------------------------------------------------------------------------------
      Test Example Data
    ----------------------------------------------------------------------------------*/

    #[test]
    fn test_get_part_numbers() {
        let input = PathBuf::from("part1_example.txt");
        let engine_schematic = read_input(&input);
        let part_numbers = get_part_numbers(&engine_schematic);

        let expected_part_numbers = vec![467, 35, 633, 617, 592, 755, 664, 598];
        assert_eq!(part_numbers, expected_part_numbers);
    }

    #[test]
    fn test_part1_example() {
        let input = PathBuf::from("part1_example.txt");
        let answer = part1(input);
        assert_eq!(answer, 4361);
    }

    #[test]
    fn test_part1_solution() {
        let input = PathBuf::from("input.txt");
        let answer = part1(input);
        assert_eq!(answer, 539637);
    }
}
