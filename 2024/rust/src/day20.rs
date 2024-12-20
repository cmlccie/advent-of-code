use crate::shared::map::{Direction4C, Map, MapIndex, Offset};
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::fs::read_to_string;
use std::path::Path;
use strum::IntoEnumIterator;

/*-------------------------------------------------------------------------------------------------
  Day 20: Race Condition
-------------------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let map = parse_input_file(input);
    count_cheats_that_save_time(&map, 2, 100).to_string()
}

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let map = parse_input_file(input);

    count_cheats_that_save_time(&map, 20, 100).to_string()
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Time = usize;
type CheatCount = usize;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Map<char> {
    read_to_string(input).unwrap().as_str().into()
}

fn count_cheats_that_save_time(
    map: &Map<char>,
    cheat_max_time: Time,
    min_savings: Time,
) -> CheatCount {
    let course = map_course(map);

    let course_index: HashMap<MapIndex, Time> = course
        .iter()
        .enumerate()
        .map(|(time, position)| (*position, time))
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

fn map_course(map: &Map<char>) -> Vec<MapIndex> {
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

fn next_position(map: &Map<char>, course: &[MapIndex]) -> MapIndex {
    let current_position = course.last().unwrap();
    let previous_position = if course.len() > 2 {
        course.get(course.len() - 2).unwrap()
    } else {
        current_position
    };
    for direction in Direction4C::iter() {
        if let Some(position) = map.project_index_direction(*current_position, direction) {
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

fn position_is_track(map: &Map<char>, position: MapIndex) -> bool {
    map.get(position) != Some(&'#')
}

/*-----------------------------------------------------------------------------
  Cheat!
-----------------------------------------------------------------------------*/

fn find_cheats(
    map: &Map<char>,
    course_index: &HashMap<MapIndex, Time>,
    start: MapIndex,
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
    let duration = isize::try_from(duration).unwrap();
    let offset_grid = duration * 2 + 1;
    let center = (offset_grid / 2, offset_grid / 2);
    (0..offset_grid)
        .cartesian_product(0..offset_grid)
        .filter_map(|offset| {
            let offset = (offset.0 - center.0, offset.1 - center.1);
            let offset_duration = offset.0.abs() + offset.1.abs();
            (offset_duration <= duration).then_some((offset, offset_duration as Time))
        })
        .filter(|(offset, _)| offset != &center)
        .collect()
}

fn reachable_positions(map: &Map<char>, start: MapIndex, duration: Time) -> Vec<(MapIndex, Time)> {
    get_offsets(duration)
        .iter()
        .filter_map(|(offset, duration)| {
            Some((map.project_index_offset(start, *offset)?, *duration as Time))
        })
        .filter(|(position, _)| position_is_track(map, *position))
        .collect()
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct Cheat {
    start: MapIndex,
    end: MapIndex,
    saves: Time,
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day20/input.txt"),
            solution("../data/day20/input-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day20/input.txt"),
            solution("../data/day20/input-part2-answer.txt")
        );
    }
}
