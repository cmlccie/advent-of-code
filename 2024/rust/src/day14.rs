use crate::get_input;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 14: Restroom Redoubt
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str, width: isize, height: isize) -> Option<String> {
    let lobby = Lobby::new(width, height);
    let mut robots = parse_input(input);

    for _ in 0..100 {
        robots.iter_mut().for_each(|robot| robot.r#move(&lobby));
    }

    let safety_factor = calculate_safety_factor(&robots, &lobby);

    Some(safety_factor.to_string())
}

pub fn part2(input: &str, visualize: bool) -> Option<String> {
    let lobby = Lobby::new(101, 103);
    let mut robots = parse_input(input);

    let mut first_christmas_tree: usize = 0;

    for second in 0.. {
        if second > 47 && ((second - 47) % 103 == 0) && ((second - 82) % 101 == 0) {
            first_christmas_tree = second;
            break;
        }

        robots.iter_mut().for_each(|robot| robot.r#move(&lobby));
    }

    if visualize {
        lobby.print(&robots);
    }

    Some(first_christmas_tree.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Position = (isize, isize);
type Velocity = (isize, isize);

fn parse_input(input: &str) -> Vec<Robot> {
    let robot_regex =
        Regex::new(r#"p=(?P<px>\d+),(?P<py>\d+)\sv=(?P<vx>-?\d+),(?P<vy>-?\d+)"#).unwrap();

    robot_regex
        .captures_iter(input)
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
    Part1 {
        input: PathBuf,

        #[clap(short, long, default_value = "101")]
        width: isize,

        #[clap(short, long, default_value = "103")]
        height: isize,
    },
    Part2 {
        input: PathBuf,

        #[clap(short, long, default_value = "false")]
        visualize: bool,
    },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 {
            input,
            width,
            height,
        } => part1(&get_input(&input), width, height),
        Args::Part2 { input, visualize } => part2(&get_input(&input), visualize),
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
    fn test_example_part1() {
        assert_eq!(
            part1(&get_input("../data/day14/example0.txt"), 11, 7),
            get_answer("../data/day14/example0-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day14/input.txt"), 101, 103),
            get_answer("../data/day14/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day14/input.txt"), false),
            get_answer("../data/day14/input-part2-answer.txt")
        );
    }
}
