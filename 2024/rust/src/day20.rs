use crate::{get_input, GridDirection, GridIndex, GridMap};
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use strum::IntoEnumIterator;

/*-------------------------------------------------------------------------------------------------
  Day 20: Race Condition
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let map = parse_input(input);
    let cheat_count = count_cheats_that_save_time(&map, 2, 100);

    Some(cheat_count.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let map = parse_input(input);

    let cheat_count = count_cheats_that_save_time(&map, 20, 100);

    Some(cheat_count.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Index = i16;
type Offset = GridIndex<Index>;

type Time = i16;
type CheatCount = usize;

fn parse_input(input: &str) -> GridMap<Index, char> {
    input.into()
}

fn count_cheats_that_save_time(
    map: &GridMap<Index, char>,
    cheat_max_time: Time,
    min_savings: Time,
) -> CheatCount {
    let course = map_course(map);

    let course_index: HashMap<GridIndex<Index>, Time> = course
        .iter()
        .enumerate()
        .map(|(time, position)| (*position, time.try_into().unwrap()))
        .collect();

    let cheats: Vec<Cheat> = course
        .iter()
        .flat_map(|start| find_cheats(map, &course_index, *start, cheat_max_time))
        .collect();

    let savings: BTreeMap<Time, CheatCount> =
        cheats
            .iter()
            .map(|cheat| cheat.saves)
            .fold(BTreeMap::new(), |mut savings, save| {
                *savings.entry(save).or_insert(0) += 1;
                savings
            });

    for (time, count) in &savings {
        log::debug!("There are {count} cheats that save {time} picoseconds.");
    }

    savings
        .iter()
        .filter_map(|(savings, count)| (*savings >= min_savings).then_some(*count))
        .sum()
}

/*-----------------------------------------------------------------------------
  Map Course
-----------------------------------------------------------------------------*/

fn map_course(map: &GridMap<Index, char>) -> Vec<GridIndex<Index>> {
    let start = map.find(|&c| c == 'S').unwrap();
    let goal = map.find(|&c| c == 'E').unwrap();
    let mut position = start;
    let mut course = Vec::new();
    while position != goal {
        course.push(position);
        position = next_position(map, &course);
    }
    course.push(goal);
    course
}

fn next_position(map: &GridMap<Index, char>, course: &[GridIndex<Index>]) -> GridIndex<Index> {
    let current_position = course.last().unwrap();
    let previous_position = if course.len() > 2 {
        course.get(course.len() - 2).unwrap()
    } else {
        current_position
    };
    for direction in GridDirection::iter() {
        if let Some(position) = map.project_direction(*current_position, direction) {
            if position == *previous_position {
                continue;
            }

            if position_is_track(map, position) {
                return position;
            }
        }
    }
    panic!("No next position found!");
}

fn position_is_track(map: &GridMap<Index, char>, position: GridIndex<Index>) -> bool {
    map.get(position) != Some(&'#')
}

/*-----------------------------------------------------------------------------
  Cheat!
-----------------------------------------------------------------------------*/

fn find_cheats(
    map: &GridMap<Index, char>,
    course_index: &HashMap<GridIndex<Index>, Time>,
    start: GridIndex<Index>,
    duration: Time,
) -> Vec<Cheat> {
    let start_time = course_index[&start];
    reachable_positions(map, start, duration)
        .iter()
        .filter_map(|(end, cheat_duration)| {
            let normal_end_time = course_index[end];
            let cheat_end_time = start_time + cheat_duration;
            (cheat_end_time < normal_end_time).then(|| Cheat {
                start,
                end: *end,
                saves: normal_end_time - cheat_end_time,
            })
        })
        .collect()
}

#[cached]
fn get_offsets(duration: Time) -> Vec<(Offset, Time)> {
    let offset_grid = duration * 2 + 1;
    let center: Offset = (offset_grid / 2, offset_grid / 2).into();
    (0..offset_grid)
        .cartesian_product(0..offset_grid)
        .filter_map(|offset| {
            let offset: Offset = offset.into();
            let offset: Offset = offset - center;
            let offset_duration: Time = offset.row.abs() + offset.column.abs();
            (offset_duration <= duration).then_some((offset, offset_duration))
        })
        .filter(|(offset, _)| offset != &center)
        .collect()
}

fn reachable_positions(
    map: &GridMap<Index, char>,
    start: GridIndex<Index>,
    duration: Time,
) -> Vec<(GridIndex<Index>, Time)> {
    get_offsets(duration)
        .iter()
        .filter_map(|(offset, duration)| {
            Some((map.project_offset(start, *offset)?, *duration as Time))
        })
        .filter(|(position, _)| position_is_track(map, *position))
        .collect()
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct Cheat {
    start: GridIndex<Index>,
    end: GridIndex<Index>,
    saves: Time,
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 20: Race Condition")]
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
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day20/input.txt")),
            get_answer("../data/day20/input-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day20/input.txt")),
            get_answer("../data/day20/input-part2-answer.txt")
        );
    }
}
