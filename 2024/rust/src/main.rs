use clap::{Parser, Subcommand};
use stderrlog::LogLevelNum;

/*-------------------------------------------------------------------------------------------------
  Advent of Code 2024
-------------------------------------------------------------------------------------------------*/

/*--------------------------------------------------------------------------------------
  Command Line Interface (CLI)
--------------------------------------------------------------------------------------*/

#[derive(Parser)]
#[command(version, about, long_about = "Advent of Code 2024")]
struct Args {
    #[arg(short, long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    day: Days,
}

#[derive(Subcommand)]
enum Days {
    #[clap(subcommand)]
    Day1(aoc24::day1::Args),
    #[clap(subcommand)]
    Day2(aoc24::day2::Args),
    #[clap(subcommand)]
    Day3(aoc24::day3::Args),
    #[clap(subcommand)]
    Day4(aoc24::day4::Args),
    #[clap(subcommand)]
    Day5(aoc24::day5::Args),
    #[clap(subcommand)]
    Day6(aoc24::day6::Args),
    #[clap(subcommand)]
    Day7(aoc24::day7::Args),
    #[clap(subcommand)]
    Day8(aoc24::day8::Args),
    #[clap(subcommand)]
    Day9(aoc24::day9::Args),
    #[clap(subcommand)]
    Day10(aoc24::day10::Args),
    #[clap(subcommand)]
    Day11(aoc24::day11::Args),
    #[clap(subcommand)]
    Day12(aoc24::day12::Args),
    #[clap(subcommand)]
    Day13(aoc24::day13::Args),
    #[clap(subcommand)]
    Day14(aoc24::day14::Args),
    #[clap(subcommand)]
    Day15(aoc24::day15::Args),
    #[clap(subcommand)]
    Day16(aoc24::day16::Args),
    #[clap(subcommand)]
    Day17(aoc24::day17::Args),
    #[clap(subcommand)]
    Day18(aoc24::day18::Args),
    #[clap(subcommand)]
    Day19(aoc24::day19::Args),
    #[clap(subcommand)]
    Day20(aoc24::day20::Args),
    #[clap(subcommand)]
    Day21(aoc24::day21::Args),
    #[clap(subcommand)]
    Day22(aoc24::day22::Args),
    #[clap(subcommand)]
    Day23(aoc24::day23::Args),
    #[clap(subcommand)]
    Day24(aoc24::day24::Args),
    #[clap(subcommand)]
    Day25(aoc24::day25::Args),
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

    let answer: Option<String> = match args.day {
        Days::Day1(args) => aoc24::day1::main(args),
        Days::Day2(args) => aoc24::day2::main(args),
        Days::Day3(args) => aoc24::day3::main(args),
        Days::Day4(args) => aoc24::day4::main(args),
        Days::Day5(args) => aoc24::day5::main(args),
        Days::Day6(args) => aoc24::day6::main(args),
        Days::Day7(args) => aoc24::day7::main(args),
        Days::Day8(args) => aoc24::day8::main(args),
        Days::Day9(args) => aoc24::day9::main(args),
        Days::Day10(args) => aoc24::day10::main(args),
        Days::Day11(args) => aoc24::day11::main(args),
        Days::Day12(args) => aoc24::day12::main(args),
        Days::Day13(args) => aoc24::day13::main(args),
        Days::Day14(args) => aoc24::day14::main(args),
        Days::Day15(args) => aoc24::day15::main(args),
        Days::Day16(args) => aoc24::day16::main(args),
        Days::Day17(args) => aoc24::day17::main(args),
        Days::Day18(args) => aoc24::day18::main(args),
        Days::Day19(args) => aoc24::day19::main(args),
        Days::Day20(args) => aoc24::day20::main(args),
        Days::Day21(args) => aoc24::day21::main(args),
        Days::Day22(args) => aoc24::day22::main(args),
        Days::Day23(args) => aoc24::day23::main(args),
        Days::Day24(args) => aoc24::day24::main(args),
        Days::Day25(args) => aoc24::day25::main(args),
    };

    if let Some(answer) = answer {
        println!("Answer: {}", answer);
    };
}
