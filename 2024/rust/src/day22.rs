use crate::shared::inputs::get_input;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::vec;

/*-------------------------------------------------------------------------------------------------
  Day 22: Monkey Market
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let secrets = parse_input(input);

    let secrets_sum: Answer = secrets
        .into_iter()
        .map(|secret| {
            let mut secret = secret;
            for _ in 0..2000 {
                secret = evolve_secret(secret);
            }
            secret as Answer
        })
        .sum();

    Some(secrets_sum.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let secrets = parse_input(input);
    let max_bananas = find_maximum_bananas(secrets);

    Some(max_bananas.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Secret = u64;
type Secrets = Vec<Secret>;
type Price = i8;
type Prices = Vec<Price>;
type PriceChange = Price;
type Sequence = [PriceChange; 4];
type Answer = u64;

fn parse_input(input: &str) -> Secrets {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn evolve_secret(secret: Secret) -> Secret {
    let secret = ((secret * 64) ^ secret) % 16777216;
    let secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

fn find_maximum_bananas(secrets: Secrets) -> Answer {
    let mut bananas_by_sequence: HashMap<Sequence, Answer> = HashMap::new();
    let mut prices: Prices = vec![0; 2000];
    let mut seller_sequences: HashSet<Sequence> = HashSet::new();

    for secret in secrets {
        // Calculate the prices for this buyer's secret
        let mut secret = secret;
        for price in prices.iter_mut() {
            *price = (secret % 10) as Price;
            secret = evolve_secret(secret);
        }

        // Identify how many bananas can be bought for each sequence of 4 price changes
        for window in prices.windows(5) {
            let sequence: Sequence = [
                window[1] - window[0],
                window[2] - window[1],
                window[3] - window[2],
                window[4] - window[3],
            ];

            // Identify how many bananas can be bought with the first occurrence of each sequence
            // from this seller
            if !seller_sequences.contains(&sequence) {
                let bananas = bananas_by_sequence.entry(sequence).or_insert(0);
                *bananas += window[4] as Answer;
                seller_sequences.insert(sequence);
            }
        }

        // Reset the seller sequences for the next seller
        seller_sequences.clear();
    }

    bananas_by_sequence.values().max().copied().unwrap()
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 22: Monkey Market")]
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
    fn test_example0_part1() {
        assert_eq!(
            part1(&get_input("../data/day22/example0.txt")),
            get_answer("../data/day22/example0-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day22/input.txt")),
            get_answer("../data/day22/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example1_part2() {
        assert_eq!(
            part2(&get_input("../data/day22/example1.txt")),
            get_answer("../data/day22/example1-part2-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day22/input.txt")),
            get_answer("../data/day22/input-part2-answer.txt")
        );
    }
}
