/*-------------------------------------------------------------------------------------------------
  Advent of Code 2024
-------------------------------------------------------------------------------------------------*/

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use stderrlog::LogLevelNum;

/*--------------------------------------------------------------------------------------
  Modules
--------------------------------------------------------------------------------------*/

mod day1;
mod day2;
mod day3;
mod utils;

/*--------------------------------------------------------------------------------------
  CLI Args
--------------------------------------------------------------------------------------*/

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pub debug: bool,

    #[clap(value_enum)]
    day: Days,

    #[clap(value_enum)]
    part: Parts,

    input: PathBuf,
}

#[derive(ValueEnum, Clone, Debug)]
enum Days {
    Day1,
    Day2,
    Day3,
}

#[derive(ValueEnum, Clone, Debug)]
enum Parts {
    Part1,
    Part2,
}

/*--------------------------------------------------------------------------------------
  Main
--------------------------------------------------------------------------------------*/

fn main() {
    let args = Args::parse();

    // Configure logging
    stderrlog::new()
        .module(module_path!())
        .verbosity(if args.debug {
            LogLevelNum::Debug
        } else {
            LogLevelNum::Info
        })
        .init()
        .unwrap();

    // Call the appropriate function to get the answer
    let answer = match args.day {
        Days::Day1 => match args.part {
            Parts::Part1 => day1::part1(&args.input),
            Parts::Part2 => day1::part2(&args.input),
        },

        Days::Day2 => match args.part {
            Parts::Part1 => day2::part1(&args.input),
            Parts::Part2 => day2::part2(&args.input),
        },

        Days::Day3 => match args.part {
            Parts::Part1 => day3::part1(&args.input),
            Parts::Part2 => day3::part2(&args.input),
        },
    };

    println!("Answer: {}", answer);
}
