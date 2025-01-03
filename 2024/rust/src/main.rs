use clap::{Parser, Subcommand};
use stderrlog::LogLevelNum;

/*-------------------------------------------------------------------------------------------------
  Advent of Code 2024
-------------------------------------------------------------------------------------------------*/

/*--------------------------------------------------------------------------------------
  Modules
--------------------------------------------------------------------------------------*/

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod shared;
mod utils;

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
    Day1(day1::Args),
    #[clap(subcommand)]
    Day2(day2::Args),
    #[clap(subcommand)]
    Day3(day3::Args),
    #[clap(subcommand)]
    Day4(day4::Args),
    #[clap(subcommand)]
    Day5(day5::Args),
    #[clap(subcommand)]
    Day6(day6::Args),
    #[clap(subcommand)]
    Day7(day7::Args),
    #[clap(subcommand)]
    Day8(day8::Args),
    #[clap(subcommand)]
    Day9(day9::Args),
    #[clap(subcommand)]
    Day10(day10::Args),
    #[clap(subcommand)]
    Day11(day11::Args),
    #[clap(subcommand)]
    Day12(day12::Args),
    #[clap(subcommand)]
    Day13(day13::Args),
    #[clap(subcommand)]
    Day14(day14::Args),
    #[clap(subcommand)]
    Day15(day15::Args),
    #[clap(subcommand)]
    Day16(day16::Args),
    #[clap(subcommand)]
    Day17(day17::Args),
    #[clap(subcommand)]
    Day18(day18::Args),
    #[clap(subcommand)]
    Day19(day19::Args),
    #[clap(subcommand)]
    Day20(day20::Args),
    #[clap(subcommand)]
    Day21(day21::Args),
    #[clap(subcommand)]
    Day22(day22::Args),
    #[clap(subcommand)]
    Day23(day23::Args),
    #[clap(subcommand)]
    Day24(day24::Args),
    #[clap(subcommand)]
    Day25(day25::Args),
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
        Days::Day1(args) => day1::main(args),
        Days::Day2(args) => day2::main(args),
        Days::Day3(args) => day3::main(args),
        Days::Day4(args) => day4::main(args),
        Days::Day5(args) => day5::main(args),
        Days::Day6(args) => day6::main(args),
        Days::Day7(args) => day7::main(args),
        Days::Day8(args) => day8::main(args),
        Days::Day9(args) => day9::main(args),
        Days::Day10(args) => day10::main(args),
        Days::Day11(args) => day11::main(args),
        Days::Day12(args) => day12::main(args),
        Days::Day13(args) => day13::main(args),
        Days::Day14(args) => day14::main(args),
        Days::Day15(args) => day15::main(args),
        Days::Day16(args) => day16::main(args),
        Days::Day17(args) => day17::main(args),
        Days::Day18(args) => day18::main(args),
        Days::Day19(args) => day19::main(args),
        Days::Day20(args) => day20::main(args),
        Days::Day21(args) => day21::main(args),
        Days::Day22(args) => day22::main(args),
        Days::Day23(args) => day23::main(args),
        Days::Day24(args) => day24::main(args),
        Days::Day25(args) => day25::main(args),
    };

    if let Some(answer) = answer {
        println!("Answer: {}", answer);
    };
}
