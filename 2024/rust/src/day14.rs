use crate::get_input;
use regex::Regex;
use std::path::PathBuf;

const Z_SCORE_THRESHOLD: f64 = 5.0;

/*-------------------------------------------------------------------------------------------------
  Day 14: Restroom Redoubt
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str, width: isize, height: isize) -> Option<String> {
    let lobby = Lobby::new(width, height);
    let robots = parse_input(input);

    let robots = move_robots(&robots, 100, &lobby);

    let safety_factor = calculate_safety_factor(&robots, &lobby);

    Some(safety_factor.to_string())
}

pub fn part2(input: &str, visualize: bool) -> Option<String> {
    let lobby = Lobby::new(101, 103);
    let robots = parse_input(input);

    let mut running_xy_mean: Option<(f64, f64)> = None;
    let mut running_xy_variance: Option<(f64, f64)> = None;

    let mut first_x_spike: Option<usize> = None;
    let mut first_y_spike: Option<usize> = None;
    let mut second_x_spike: Option<usize> = None;
    let mut second_y_spike: Option<usize> = None;

    for second in 0..1000 {
        let robots = move_robots(&robots, second, &lobby);

        let xy_mean = calculate_xy_mean(&robots);
        let xy_variance = calculate_xy_variance(&robots, xy_mean);

        running_xy_mean = update_running_mean(running_xy_mean, xy_mean, second + 1);
        running_xy_variance = update_running_mean(running_xy_variance, xy_variance, second + 1);

        let running_xy_variance_standard_deviation =
            calculate_xy_standard_deviation(running_xy_variance.unwrap());

        let xy_variance_z_score = calculate_xy_z_score(
            xy_variance,
            running_xy_variance.unwrap(),
            running_xy_variance_standard_deviation,
        );

        let (x_spike, y_spike) = (
            xy_variance_z_score.0 > Z_SCORE_THRESHOLD,
            xy_variance_z_score.1 > Z_SCORE_THRESHOLD,
        );

        (first_x_spike, second_x_spike) = match (x_spike, first_x_spike, second_x_spike) {
            (true, None, None) => (Some(second), None),
            (true, Some(_), None) => (first_x_spike, Some(second)),
            _ => (first_x_spike, second_x_spike),
        };

        (first_y_spike, second_y_spike) = match (y_spike, first_y_spike, second_y_spike) {
            (true, None, None) => (Some(second), None),
            (true, Some(_), None) => (first_y_spike, Some(second)),
            _ => (first_y_spike, second_y_spike),
        };

        if first_x_spike.is_some()
            && second_x_spike.is_some()
            && first_y_spike.is_some()
            && second_y_spike.is_some()
        {
            break;
        }
    }

    let first_christmas_tree = find_cycle_intersection(
        first_x_spike.unwrap(),
        second_x_spike.unwrap(),
        first_y_spike.unwrap(),
        second_y_spike.unwrap(),
    )
    .unwrap();

    if visualize {
        let robots = move_robots(&robots, first_christmas_tree, &lobby);
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

fn move_robots(robots: &[Robot], steps: usize, lobby: &Lobby) -> Vec<Robot> {
    robots
        .iter()
        .map(|robot| {
            let new_position = robot.new_position(steps, lobby);
            Robot::new(new_position, robot.velocity)
        })
        .collect()
}

fn calculate_safety_factor(robots: &[Robot], lobby: &Lobby) -> usize {
    let mut counts = [0; 4];

    robots
        .iter()
        .filter_map(|robot| lobby.quadrant(robot.position))
        .for_each(|quadrant| counts[quadrant as usize] += 1);

    counts.iter().product()
}

fn calculate_xy_mean(robots: &[Robot]) -> (f64, f64) {
    let n = robots.len() as f64;
    let (sum_x, sum_y) = robots.iter().fold((0.0, 0.0), |(sx, sy), robot| {
        let (x, y) = robot.position;
        (sx + x as f64, sy + y as f64)
    });
    (sum_x / n, sum_y / n)
}

fn calculate_xy_variance(robots: &[Robot], xy_mean: (f64, f64)) -> (f64, f64) {
    let n = robots.len() as f64;
    let (mean_x, mean_y) = xy_mean;

    let (var_x, var_y) = robots.iter().fold((0.0, 0.0), |(vx, vy), robot| {
        let (x, y) = robot.position;
        (
            vx + (x as f64 - mean_x).powi(2),
            vy + (y as f64 - mean_y).powi(2),
        )
    });

    (var_x / n, var_y / n)
}

fn calculate_xy_standard_deviation(variance: (f64, f64)) -> (f64, f64) {
    (variance.0.sqrt(), variance.1.sqrt())
}

fn calculate_xy_z_score(
    value: (f64, f64),
    mean: (f64, f64),
    standard_deviation: (f64, f64),
) -> (f64, f64) {
    (
        (value.0 - mean.0).abs() / standard_deviation.0,
        (value.1 - mean.1).abs() / standard_deviation.1,
    )
}

fn update_running_mean(
    mean: Option<(f64, f64)>,
    new_value: (f64, f64),
    count: usize,
) -> Option<(f64, f64)> {
    if mean.is_none() {
        return Some(new_value);
    }

    let current_mean = mean.unwrap();

    Some((
        current_mean.0 + (new_value.0 - current_mean.0) / count as f64,
        current_mean.1 + (new_value.1 - current_mean.1) / count as f64,
    ))
}

fn find_cycle_intersection(
    first_x_spike: usize,
    second_x_spike: usize,
    first_y_spike: usize,
    second_y_spike: usize,
) -> Option<usize> {
    let x_period = second_x_spike - first_x_spike;
    let y_period = second_y_spike - first_y_spike;
    let phase_diff = first_y_spike as isize - first_x_spike as isize;

    // Solve: first_x_spike + n1 * x_period = first_y_spike + n2 * y_period
    // Use extended Euclidean algorithm to find solution
    // ax + by = gcd(a, b)
    let (gcd, x, _y) = extended_euclidean(x_period as isize, -(y_period as isize));

    // Check solution exists
    if phase_diff % gcd != 0 {
        return None;
    }

    let scale = phase_diff / gcd;
    let n1 = x * scale;
    let intersection = first_x_spike as isize + n1 * x_period as isize;

    // Ensure positive result
    if intersection >= 0 {
        Some(intersection as usize)
    } else {
        // Find the next positive occurrence using LCM
        let lcm = (x_period * y_period) / gcd as usize;
        let positive_intersection =
            intersection + ((-intersection / lcm as isize + 1) * lcm as isize);
        Some(positive_intersection as usize)
    }
}

fn extended_euclidean(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = extended_euclidean(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }
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

    fn new_position(&self, steps: usize, lobby: &Lobby) -> Position {
        let (x, y) = self.position;
        let (vx, vy) = self.velocity;
        (
            ((x + vx * steps as isize) % lobby.width + lobby.width) % lobby.width,
            ((y + vy * steps as isize) % lobby.height + lobby.height) % lobby.height,
        )
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
    Q1 = 0,
    Q2 = 1,
    Q3 = 2,
    Q4 = 3,
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
