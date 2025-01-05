use crate::shared::inputs::get_input;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 4: Ceres Search
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let word = Word::new("XMAS");

    let puzzle: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word_search = WordSearch::new(puzzle);

    let xmas_count = WordSearchIterator::new(&word_search)
        .filter(|(letter, _)| *letter == 'X')
        .map(|(_, start)| word_search.find_word(&start, &word))
        .sum::<i64>();

    Some(xmas_count.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let word = Xmas::new();

    let puzzle: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word_search = WordSearch::new(puzzle);

    let xmas_count = WordSearchIterator::new(&word_search)
        .filter(|(letter, _)| *letter == 'A')
        .filter_map(|(_, start)| word_search.find_x_mas(&start, &word))
        .sum::<i64>();

    Some(xmas_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
  Word Search
-----------------------------------------------------------------------------*/

struct WordSearch {
    puzzle: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
}

impl WordSearch {
    fn new(puzzle: Vec<Vec<char>>) -> Self {
        let rows = puzzle.len();
        let columns = puzzle[0].len();
        Self {
            puzzle,
            rows,
            columns,
        }
    }

    fn get_letter(&self, row: usize, column: usize) -> Option<char> {
        if row < self.rows && column < self.columns {
            Some(self.puzzle[row][column])
        } else {
            None
        }
    }

    fn project(&self, coordinate: &(usize, usize), offset: &(i32, i32)) -> Option<(usize, usize)> {
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

    fn project_offsets(
        &self,
        start: &(usize, usize),
        offsets: &[(i32, i32)],
    ) -> Option<Vec<(usize, usize)>> {
        let mut coordinates = Vec::new();

        for offset in offsets {
            coordinates.push(self.project(start, offset)?);
        }

        Some(coordinates)
    }
}

/*-----------------------------------------------------------------------------
  Word Search Iterator
-----------------------------------------------------------------------------*/

struct WordSearchIterator<'a> {
    word_search: &'a WordSearch,
    row: usize,
    column: usize,
}

impl<'a> WordSearchIterator<'a> {
    fn new(word_search: &'a WordSearch) -> Self {
        Self {
            word_search,
            row: 0,
            column: 0,
        }
    }
}

impl Iterator for WordSearchIterator<'_> {
    type Item = (char, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        // Get the letter at the current position
        let letter = self.word_search.get_letter(self.row, self.column)?;
        let value = (letter, (self.row, self.column));

        // Advance the iterator to the next position
        let next_column = self.column + 1;
        (self.row, self.column) = if next_column == self.word_search.columns {
            (self.row + 1, 0)
        } else {
            (self.row, next_column)
        };

        Some(value)
    }
}

/*-----------------------------------------------------------------------------
  Word
-----------------------------------------------------------------------------*/

struct Word {
    word: String,
    offsets: [Vec<(i32, i32)>; 8],
}

impl Word {
    fn new(word: &str) -> Self {
        let word_length = word.len();

        let mut offsets: [Vec<(i32, i32)>; 8] = [
            Vec::with_capacity(word_length),
            Vec::with_capacity(word_length),
            Vec::with_capacity(word_length),
            Vec::with_capacity(word_length),
            Vec::with_capacity(word_length),
            Vec::with_capacity(word_length),
            Vec::with_capacity(word_length),
            Vec::with_capacity(word_length),
        ];

        for index in 0..word_length {
            let index = index as i32;
            offsets[0].push((-index, -index)); // North West
            offsets[1].push((-index, 0)); // North
            offsets[2].push((-index, index)); // North East
            offsets[3].push((0, index)); // East
            offsets[4].push((index, index)); // South East
            offsets[5].push((index, 0)); // South
            offsets[6].push((index, -index)); // South West
            offsets[7].push((0, -index)); // West
        }

        Self {
            word: word.to_string(),
            offsets,
        }
    }
}

/*-----------------------------------------------------------------------------
  Word Search Methods
-----------------------------------------------------------------------------*/

impl WordSearch {
    fn find_word(&self, start: &(usize, usize), word: &Word) -> i64 {
        word.offsets
            .iter()
            .filter_map(|offsets| self.project_offsets(start, offsets))
            .map(|coordinates| self.check_word(&coordinates, &word.word))
            .filter(|found| *found)
            .count() as i64
    }

    fn check_word(&self, coordinates: &[(usize, usize)], word: &str) -> bool {
        assert!(coordinates.len() == word.len());
        coordinates
            .iter()
            .enumerate()
            .all(|(index, (row, column))| {
                self.get_letter(*row, *column) == Some(word.chars().nth(index).unwrap())
            })
    }
}

/*-----------------------------------------------------------------------------
  Xmas Word
-----------------------------------------------------------------------------*/

struct Xmas {
    offsets: [(i32, i32); 5],
}

impl Xmas {
    fn new() -> Self {
        Self {
            offsets: [
                (-1, -1), // North West
                (-1, 1),  // North East
                (0, 0),   // Center
                (1, -1),  // South West
                (1, 1),   // South East
            ],
        }
    }
}

/*-----------------------------------------------------------------------------
  Word Search Methods
-----------------------------------------------------------------------------*/

impl WordSearch {
    fn find_x_mas(&self, start: &(usize, usize), xmas: &Xmas) -> Option<i64> {
        let coordinates = self.project_offsets(start, &xmas.offsets)?;
        let letters: [char; 5] = [
            self.get_letter(coordinates[0].0, coordinates[0].1)?,
            self.get_letter(coordinates[1].0, coordinates[1].1)?,
            self.get_letter(coordinates[2].0, coordinates[2].1)?,
            self.get_letter(coordinates[3].0, coordinates[3].1)?,
            self.get_letter(coordinates[4].0, coordinates[4].1)?,
        ];

        match letters {
            ['M', 'M', 'A', 'S', 'S'] => Some(1),
            ['M', 'S', 'A', 'M', 'S'] => Some(1),
            ['S', 'M', 'A', 'S', 'M'] => Some(1),
            ['S', 'S', 'A', 'M', 'M'] => Some(1),
            _ => None,
        }
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 4: Ceres Search")]
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
    fn test_example_solution_part1() {
        assert_eq!(
            part1(&get_input("../data/day4/example.txt")),
            get_answer("../data/day4/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day4/example.txt")),
            get_answer("../data/day4/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day4/input.txt")),
            get_answer("../data/day4/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day4/input.txt")),
            get_answer("../data/day4/input-part2-answer.txt")
        );
    }
}
