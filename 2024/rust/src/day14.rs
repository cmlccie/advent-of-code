use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/*-------------------------------------------------------------------------------------------------
  Day 14: Restroom Redoubt
-------------------------------------------------------------------------------------------------*/

fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let lobby = Lobby::new(101, 103);
    let mut robots = parse_input_file(input);

    for _ in 0..100 {
        robots.iter_mut().for_each(|robot| robot.r#move(&lobby));
    }

    let safety_factor = calculate_safety_factor(&robots, &lobby);

    Some(safety_factor.to_string())
}

fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let lobby = Lobby::new(101, 103);
    let mut robots = parse_input_file(input);

    let mut first_christmas_tree: usize = 0;

    for second in 0.. {
        if second > 47 && ((second - 47) % 103 == 0) && ((second - 82) % 101 == 0) {
            // print!("\x1b[104A");
            println!("Second: {}", second);
            lobby.print(&robots);
            first_christmas_tree = second;
            break;
        }

        robots.iter_mut().for_each(|robot| robot.r#move(&lobby));
    }

    Some(first_christmas_tree.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Position = (isize, isize);
type Velocity = (isize, isize);

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Vec<Robot> {
    let input = read_to_string(input).unwrap();

    let robot_regex =
        Regex::new(r#"p=(?P<px>\d+),(?P<py>\d+)\sv=(?P<vx>-?\d+),(?P<vy>-?\d+)"#).unwrap();

    robot_regex
        .captures_iter(&input)
        .map(|cap| {
            Robot::new(
                (cap["px"].parse().unwrap(), cap["py"].parse().unwrap()),
                (cap["vx"].parse().unwrap(), cap["vy"].parse().unwrap()),
            )
        })
        .collect()
}

fn calculate_safety_factor(robots: &[Robot], lobby: &Lobby) -> usize {
    let mut counts: HashMap<Quadrant, usize> = HashMap::new();

    robots
        .iter()
        .filter_map(|robot| lobby.quadrant(robot.position))
        .for_each(|quadrant| {
            *counts.entry(quadrant).or_insert(0) += 1;
        });

    counts.values().product()
}

/*-----------------------------------------------------------------------------
  Robot
-----------------------------------------------------------------------------*/

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }

    fn r#move(&mut self, lobby: &Lobby) {
        let width = lobby.width;
        let height = lobby.height;

        let (x, y) = self.position;
        let (vx, vy) = self.velocity;

        let next_x = x + vx;
        let next_x = if next_x < 0 {
            width + next_x
        } else if next_x >= width {
            next_x % width
        } else {
            next_x
        };

        let next_y = y + vy;
        let next_y = if next_y < 0 {
            height + next_y
        } else if next_y >= height {
            next_y % height
        } else {
            next_y
        };

        self.position = (next_x, next_y);
    }
}

/*-----------------------------------------------------------------------------
  Lobby
-----------------------------------------------------------------------------*/

struct Lobby {
    width: isize,
    height: isize,

    q1: (Position, Position),
    q2: (Position, Position),
    q3: (Position, Position),
    q4: (Position, Position),
}

impl Lobby {
    fn new(width: isize, height: isize) -> Self {
        Self {
            width,
            height,
            q1: ((0, 0), (width / 2, height / 2)),
            q2: ((width / 2 + 1, 0), (width, height / 2)),
            q3: ((0, height / 2 + 1), (width / 2, height)),
            q4: ((width / 2 + 1, height / 2 + 1), (width, height)),
        }
    }

    fn quadrant(&self, position: Position) -> Option<Quadrant> {
        match position {
            pos if self.is_in_quadrant(pos, self.q1) => Some(Quadrant::Q1),
            pos if self.is_in_quadrant(pos, self.q2) => Some(Quadrant::Q2),
            pos if self.is_in_quadrant(pos, self.q3) => Some(Quadrant::Q3),
            pos if self.is_in_quadrant(pos, self.q4) => Some(Quadrant::Q4),
            _ => None,
        }
    }

    fn is_in_quadrant(&self, position: Position, quadrant: (Position, Position)) -> bool {
        let (x, y) = position;
        let ((x1, y1), (x2, y2)) = quadrant;
        x >= x1 && x < x2 && y >= y1 && y < y2
    }

    fn print(&self, robots: &[Robot]) {
        let map: Vec<String> = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| {
                        robots
                            .iter()
                            .find(|robot| robot.position == (x, y))
                            .map(|_| '*')
                            .unwrap_or(' ')
                    })
                    .collect()
            })
            .collect();
        for line in map {
            println!("{}", line);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Quadrant {
    Q1,
    Q2,
    Q3,
    Q4,
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 14: Restroom Redoubt")]
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
    use crate::shared::answers::answer;

    // #[test]
    // fn test_example_part1() {
    //     assert_eq!(
    //         part1("../data/day14/example0.txt"),
    //         solution("../data/day14/example0-part1-answer.txt")
    //     );
    // }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day14/input.txt"),
            answer("../data/day14/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day14/input.txt"),
            answer("../data/day14/input-part2-answer.txt")
        );
    }
}
