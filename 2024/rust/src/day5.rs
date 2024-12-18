/*-------------------------------------------------------------------------------------------------
  Day 5: Print Queue
-------------------------------------------------------------------------------------------------*/

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

/*--------------------------------------------------------------------------------------
  Helpers
--------------------------------------------------------------------------------------*/

type Updates = Vec<u8>;

/*-----------------------------------------------------------------------------
  Ordering Rules
-----------------------------------------------------------------------------*/

struct OrderingRules {
    rules: HashMap<u8, HashSet<u8>>,
}

impl OrderingRules {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn insert(&mut self, page: u8, less_than: u8) {
        self.rules.entry(page).or_default().insert(less_than);
    }

    fn less_than(&self, a: u8, b: u8) -> Ordering {
        if let Some(rules) = self.rules.get(&a) {
            if rules.contains(&b) {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

/*-----------------------------------------------------------------------------
  Sort Pages
-----------------------------------------------------------------------------*/

fn sort_pages(pages: &Updates, ordering_rules: &OrderingRules) -> Updates {
    let mut sorted_pages = pages.clone();
    sorted_pages.sort_by(|a, b| ordering_rules.less_than(*a, *b));
    sorted_pages
}

/*-----------------------------------------------------------------------------
  Parse Input File
-----------------------------------------------------------------------------*/

fn parse_input<P: AsRef<Path> + ?Sized>(input: &P) -> (OrderingRules, Vec<Updates>) {
    let contents = read_to_string(input).unwrap();
    let mut lines = contents.lines();

    let mut ordering_rules = OrderingRules::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut pages = line.split('|');
        ordering_rules.insert(
            pages.next().unwrap().parse::<u8>().unwrap(),
            pages.next().unwrap().parse::<u8>().unwrap(),
        );
    }

    let mut updates: Vec<Updates> = Vec::new();
    for line in lines.by_ref() {
        let pages: Vec<_> = line
            .split(',')
            .map(|page| page.parse::<u8>().unwrap())
            .collect();
        updates.push(pages);
    }

    (ordering_rules, updates)
}

/*--------------------------------------------------------------------------------------
  Part 1
--------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let (ordering_rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| validate_page_order(&ordering_rules, update))
        .map(|update| update[update.len() / 2] as i64)
        .sum::<i64>()
        .to_string()
}

/*-----------------------------------------------------------------------------
  Validate Page Order
-----------------------------------------------------------------------------*/

fn validate_page_order(ordering_rules: &OrderingRules, update: &Updates) -> bool {
    let sorted_update = sort_pages(update, ordering_rules);
    update == &sorted_update
}

/*--------------------------------------------------------------------------------------
  Part 2
--------------------------------------------------------------------------------------*/

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let (ordering_rules, updates) = parse_input(input);

    updates
        .iter()
        .filter_map(|update| corrected_update(&ordering_rules, update))
        .map(|update| update[update.len() / 2] as i64)
        .sum::<i64>()
        .to_string()
}

/*-----------------------------------------------------------------------------
  Corrected Updates
-----------------------------------------------------------------------------*/

fn corrected_update(ordering_rules: &OrderingRules, update: &Updates) -> Option<Updates> {
    let sorted_update = sort_pages(update, ordering_rules);
    if update == &sorted_update {
        None
    } else {
        Some(sorted_update)
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1("../data/day5/example.txt"),
            solution("../data/day5/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2("../data/day5/example.txt"),
            solution("../data/day5/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day5/input.txt"),
            solution("../data/day5/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day5/input.txt"),
            solution("../data/day5/input-part2-answer.txt")
        );
    }
}
