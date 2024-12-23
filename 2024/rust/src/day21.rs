use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

/*-------------------------------------------------------------------------------------------------
  Day 21: Keypad Conundrum
-------------------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let codes = parse_input_file(input);

    let numpad = Box::new(Keypad::new(KeypadType::NumPad, "Robot0", None));
    let dpad1 = Box::new(Keypad::new(KeypadType::DPad, "Robot1", Some(numpad)));
    let mut dpad2 = Box::new(Keypad::new(KeypadType::DPad, "Robot2", Some(dpad1)));

    codes
        .iter()
        .map(|code| (code, dpad2.enter_code(code)))
        .map(|(code, moves)| calculate_complexity(code, &moves))
        .sum::<Complexity>()
        .to_string()
}

pub fn part2<P: AsRef<Path> + ?Sized>(_input: &P) -> String {
    unimplemented!()
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Code = String;
type Position = (isize, isize);
type Complexity = usize;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Vec<Code> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

/*-----------------------------------------------------------------------------
  Keypad
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy)]
enum KeypadType {
    NumPad,
    DPad,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Keypad {
    keypad: KeypadType,
    name: String,
    connected: Option<Box<Keypad>>,
    keys: HashMap<char, Position>,
    position: Position,
}

impl Keypad {
    fn new(keypad: KeypadType, name: &str, connected: Option<Box<Keypad>>) -> Self {
        match keypad {
            KeypadType::NumPad => assert!(connected.is_none()),
            KeypadType::DPad => assert!(connected.is_some()),
        }

        let keys: HashMap<char, Position> = match keypad {
            KeypadType::NumPad => [
                ('7', (0, 0)),
                ('8', (0, 1)),
                ('9', (0, 2)),
                ('4', (1, 0)),
                ('5', (1, 1)),
                ('6', (1, 2)),
                ('1', (2, 0)),
                ('2', (2, 1)),
                ('3', (2, 2)),
                ('X', (3, 0)),
                ('0', (3, 1)),
                ('A', (3, 2)),
            ]
            .into_iter()
            .collect(),
            KeypadType::DPad => [
                ('X', (0, 0)),
                ('^', (0, 1)),
                ('A', (0, 2)),
                ('<', (1, 0)),
                ('v', (1, 1)),
                ('>', (1, 2)),
            ]
            .into_iter()
            .collect(),
        };

        let activate_key = *keys.get(&'A').unwrap();

        Keypad {
            keypad,
            name: name.to_string(),
            connected,
            keys,
            position: activate_key,
        }
    }

    fn enter_code(&mut self, code: &Code) -> Vec<Move> {
        let moves: Vec<Move> = match self.keypad {
            KeypadType::NumPad => code.chars().flat_map(|c| self.move_to(c)).collect(),
            KeypadType::DPad => self
                .connected
                .as_mut()
                .unwrap()
                .enter_code(code)
                .iter()
                .map(|m| char::from(*m))
                .flat_map(|c| self.move_to(c))
                .collect(),
        };

        log::debug!(
            "{code}: {} ({})",
            moves.iter().map(|m| char::from(*m)).collect::<String>(),
            self.name,
        );

        moves
    }

    fn move_to(&mut self, to: char) -> Vec<Move> {
        let target = *self.keys.get(&to).unwrap();
        match self.keypad {
            KeypadType::NumPad => self.numpad_select(target),
            KeypadType::DPad => self.dpad_select(target),
        }
    }

    fn dpad_select(&mut self, target: Position) -> Vec<Move> {
        let mut moves = Vec::new();

        while self.position != target {
            // Move right before moving up; Move down before moving left
            if self.position.1 < target.1 {
                moves.push(Move::Right);
                self.position.1 += 1;
            } else if self.position.0 > target.0 {
                moves.push(Move::Up);
                self.position.0 -= 1;
            } else if self.position.0 < target.0 {
                moves.push(Move::Down);
                self.position.0 += 1;
            } else if self.position.1 > target.1 {
                moves.push(Move::Left);
                self.position.1 -= 1;
            }
        }
        moves.push(Move::Activate);

        moves
    }

    fn numpad_select(&mut self, target: Position) -> Vec<Move> {
        let mut moves = Vec::new();

        while self.position != target {
            // Move right before moving down; Move up before moving left
            if self.position.1 < target.1 {
                moves.push(Move::Right);
                self.position.1 += 1;
            } else if self.position.0 < target.0 {
                moves.push(Move::Down);
                self.position.0 += 1;
            } else if self.position.0 > target.0 {
                moves.push(Move::Up);
                self.position.0 -= 1;
            } else if self.position.1 > target.1 {
                moves.push(Move::Left);
                self.position.1 -= 1;
            }
        }
        moves.push(Move::Activate);

        moves
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl From<Move> for char {
    fn from(m: Move) -> Self {
        match m {
            Move::Up => '^',
            Move::Down => 'v',
            Move::Left => '<',
            Move::Right => '>',
            Move::Activate => 'A',
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            'A' => Move::Activate,
            _ => panic!("Invalid move: {}", c),
        }
    }
}

/*-----------------------------------------------------------------------------
  Calculate Code Complexity
-----------------------------------------------------------------------------*/

fn calculate_complexity(code: &str, moves: &[Move]) -> Complexity {
    let code_value: Complexity = code[..code.len() - 1].parse().unwrap();
    let move_count: Complexity = moves.len();

    code_value * move_count
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            part1("../data/day21/example.txt"),
            solution("../data/day21/example-part1-answer.txt")
        );
    }

    // #[test]
    // fn test_part1_solution() {
    //     assert_eq!(
    //         part1("../data/day21/input.txt"),
    //         solution("../data/day21/input-part1-answer.txt")
    //     );
    // }

    // #[test]
    // fn test_part2_solution() {
    //     assert_eq!(
    //         part2("../data/day21/input.txt"),
    //         solution("../data/day21/input-part2-answer.txt")
    //     );
    // }
}
