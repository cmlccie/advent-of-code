use cached::proc_macro::cached;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::OnceLock;

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
        .map(|(code, moves)| calculate_complexity(code, moves.len()))
        .sum::<Complexity>()
        .to_string()
}

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let codes = parse_input_file(input);

    let mut previous_keypad = None;
    for n in (0..25).rev() {
        previous_keypad = Some(Box::new(Keypad::new(
            KeypadType::DPad,
            format!("Robot{n}").as_str(),
            previous_keypad,
        )));
    }

    let mut final_keypad = Box::new(Keypad::new(KeypadType::NumPad, "Robot", previous_keypad));

    codes
        .iter()
        .map(|code| {
            let move_count: MoveCount = code
                .chars()
                .map(|button| final_keypad.count_moves(button))
                .sum();
            calculate_complexity(code, move_count)
        })
        .sum::<Complexity>()
        .to_string()
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Code = String;
type Button = char;
type Position = (isize, isize);
type Keys = HashMap<Button, Position>;
type Moves = Vec<Move>;
type Complexity = usize;
type MoveCount = Complexity;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    robot_pointing_at: Button,

    move_count_cache: HashMap<(Button, Button), MoveCount>,
}

static NUMERIC_KEYS: OnceLock<Keys> = OnceLock::new();
fn numeric_keys() -> &'static Keys {
    NUMERIC_KEYS.get_or_init(|| {
        [
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
        .collect()
    })
}

static DPAD_KEYS: OnceLock<Keys> = OnceLock::new();
fn dpad_keys() -> &'static Keys {
    DPAD_KEYS.get_or_init(|| {
        [
            ('X', (0, 0)),
            ('^', (0, 1)),
            ('A', (0, 2)),
            ('<', (1, 0)),
            ('v', (1, 1)),
            ('>', (1, 2)),
        ]
        .into_iter()
        .collect()
    })
}

impl Keypad {
    fn new(keypad: KeypadType, name: &str, connected: Option<Box<Keypad>>) -> Self {
        Keypad {
            keypad,
            name: name.to_string(),
            connected,
            robot_pointing_at: 'A',
            move_count_cache: HashMap::new(),
        }
    }

    fn enter_code(&mut self, code: &Code) -> Moves {
        let moves: Moves = match self.keypad {
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

    fn move_to(&mut self, to: Button) -> Moves {
        let moves = match self.keypad {
            KeypadType::NumPad => numeric_moves(self.robot_pointing_at, to),
            KeypadType::DPad => dpad_moves(self.robot_pointing_at, to),
        };

        self.robot_pointing_at = to;

        moves
    }

    fn count_moves(&mut self, to: Button) -> MoveCount {
        // Check cache
        if let Some(&count) = self.move_count_cache.get(&(self.robot_pointing_at, to)) {
            self.robot_pointing_at = to;
            return count;
        }

        // Moves needed at this keypad
        let moves = match self.keypad {
            KeypadType::NumPad => numeric_moves(self.robot_pointing_at, to),
            KeypadType::DPad => dpad_moves(self.robot_pointing_at, to),
        };

        // Base case
        let move_count = if self.connected.is_none() {
            moves.len()
        } else {
            moves
                .into_iter()
                .map(Button::from)
                .map(|button| self.connected.as_mut().unwrap().count_moves(button))
                .sum()
        };

        self.move_count_cache
            .insert((self.robot_pointing_at, to), move_count);

        self.robot_pointing_at = to;
        move_count
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
  Moves
-----------------------------------------------------------------------------*/

#[cached]
fn numeric_moves(current: Button, next: Button) -> Moves {
    let start = *numeric_keys().get(&current).unwrap();
    let end = *numeric_keys().get(&next).unwrap();

    match (current, next) {
        ('0' | 'A', '1' | '4' | '7') | ('1' | '4' | '7', '0' | 'A') => {
            right_down_up_left(start, end)
        }
        _ => left_up_down_right(start, end),
    }
}

#[cached]
fn dpad_moves(current: Button, next: Button) -> Moves {
    let start = *dpad_keys().get(&current).unwrap();
    let end = *dpad_keys().get(&next).unwrap();

    match (current, next) {
        ('<', _) | (_, '<') => right_down_up_left(start, end),
        _ => left_up_down_right(start, end),
    }
}

/*-----------------------------------------------------------------------------
  Move Strategies
-----------------------------------------------------------------------------*/

fn left_up_down_right(start: Position, end: Position) -> Moves {
    let mut moves = Vec::new();
    let mut current = start;

    while current != end {
        if current.1 > end.1 {
            moves.push(Move::Left);
            current.1 -= 1;
        } else if current.0 > end.0 {
            moves.push(Move::Up);
            current.0 -= 1;
        } else if current.0 < end.0 {
            moves.push(Move::Down);
            current.0 += 1;
        } else {
            moves.push(Move::Right);
            current.1 += 1;
        }
    }

    moves.push(Move::Activate);

    moves
}

fn right_down_up_left(start: Position, end: Position) -> Moves {
    let mut moves = Vec::new();
    let mut current = start;

    while current != end {
        if current.1 < end.1 {
            moves.push(Move::Right);
            current.1 += 1;
        } else if current.0 < end.0 {
            moves.push(Move::Down);
            current.0 += 1;
        } else if current.0 > end.0 {
            moves.push(Move::Up);
            current.0 -= 1;
        } else {
            moves.push(Move::Left);
            current.1 -= 1;
        }
    }

    moves.push(Move::Activate);

    moves
}

/*-----------------------------------------------------------------------------
  Calculate Code Complexity
-----------------------------------------------------------------------------*/

fn calculate_complexity(code: &str, move_count: MoveCount) -> Complexity {
    let code_value: Complexity = code[..code.len() - 1].parse().unwrap();
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

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day21/input.txt"),
            solution("../data/day21/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day21/input.txt"),
            solution("../data/day21/input-part2-answer.txt")
        );
    }
}
