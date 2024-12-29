use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;
use std::vec;

/*-------------------------------------------------------------------------------------------------
  Day 22: Monkey Market
-------------------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let secrets = parse_input_file(input);

    secrets
        .into_iter()
        .map(|secret| {
            let mut secret = secret;
            for _ in 0..2000 {
                secret = evolve_secret(secret);
            }
            secret as Answer
        })
        .sum::<Answer>()
        .to_string()
}

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let secrets = parse_input_file(input);
    find_maximum_bananas(secrets).to_string()
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

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> Secrets {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
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

        log::debug!("Prices: {:?}", prices[0..10].to_vec());

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

                if sequence == [-2, 1, -1, 3] {
                    log::debug!("Seller sequence {sequence:?} buys {} bananas", window[4]);
                };

                seller_sequences.insert(sequence);
            }
        }

        // Reset the seller sequences for the next seller
        seller_sequences.clear();
    }

    bananas_by_sequence.values().max().copied().unwrap()
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_example0_part1() {
        assert_eq!(
            part1("../data/day22/example0.txt"),
            solution("../data/day22/example0-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day22/input.txt"),
            solution("../data/day22/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example1_part2() {
        assert_eq!(
            part2("../data/day22/example1.txt"),
            solution("../data/day22/example1-part2-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day22/input.txt"),
            solution("../data/day22/input-part2-answer.txt")
        );
    }
}
