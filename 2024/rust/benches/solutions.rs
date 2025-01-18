use aoc24::get_input;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/*-------------------------------------------------------------------------------------------------
  Solution Benchmarks
-------------------------------------------------------------------------------------------------*/

/*--------------------------------------------------------------------------------------
  Macro
--------------------------------------------------------------------------------------*/

macro_rules! bench_solution {
    ($c:ident, $solution:path, $input:literal) => {
        let input = get_input($input);

        $c.bench_function(stringify!($solution), |b| {
            b.iter(|| $solution(black_box(&input)));
        });
    };

    ($c:ident, $solution:path, $input:literal, $($args:expr),*) => {
        let input = get_input($input);

        $c.bench_function(stringify!($solution), |b| {
            b.iter(|| $solution(black_box(&input), $($args),*));
        });
    };
}

/*--------------------------------------------------------------------------------------
  Benchmarks
--------------------------------------------------------------------------------------*/

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    // Benchmark all solutions
    bench_solution!(c, aoc24::day1::part1, "../data/day1/input.txt");
    bench_solution!(c, aoc24::day1::part2, "../data/day1/input.txt");
    bench_solution!(c, aoc24::day2::part1, "../data/day2/input.txt");
    bench_solution!(c, aoc24::day2::part2, "../data/day2/input.txt");
    bench_solution!(c, aoc24::day3::part1, "../data/day3/input.txt");
    bench_solution!(c, aoc24::day3::part2, "../data/day3/input.txt");
    bench_solution!(c, aoc24::day4::part1, "../data/day4/input.txt");
    bench_solution!(c, aoc24::day4::part2, "../data/day4/input.txt");
    bench_solution!(c, aoc24::day5::part1, "../data/day5/input.txt");
    bench_solution!(c, aoc24::day5::part2, "../data/day5/input.txt");
    bench_solution!(c, aoc24::day6::part1, "../data/day6/input.txt");
    bench_solution!(c, aoc24::day6::part2, "../data/day6/input.txt");
    bench_solution!(c, aoc24::day7::part1, "../data/day7/input.txt");
    bench_solution!(c, aoc24::day7::part2, "../data/day7/input.txt");
    bench_solution!(c, aoc24::day8::part1, "../data/day8/input.txt");
    bench_solution!(c, aoc24::day8::part2, "../data/day8/input.txt");
    bench_solution!(c, aoc24::day9::part1, "../data/day9/input.txt");
    bench_solution!(c, aoc24::day9::part2, "../data/day9/input.txt");
    bench_solution!(c, aoc24::day10::part1, "../data/day10/input.txt");
    bench_solution!(c, aoc24::day10::part2, "../data/day10/input.txt");
    bench_solution!(c, aoc24::day11::part1, "../data/day11/input.txt");
    bench_solution!(c, aoc24::day11::part2, "../data/day11/input.txt");
    bench_solution!(c, aoc24::day12::part1, "../data/day12/input.txt");
    bench_solution!(c, aoc24::day12::part2, "../data/day12/input.txt");
    bench_solution!(c, aoc24::day13::part1, "../data/day13/input.txt");
    bench_solution!(c, aoc24::day13::part2, "../data/day13/input.txt");
    bench_solution!(c, aoc24::day14::part1, "../data/day14/input.txt", 101, 103);
    bench_solution!(c, aoc24::day14::part2, "../data/day14/input.txt", false);
    bench_solution!(c, aoc24::day15::part1, "../data/day15/input.txt");
    bench_solution!(c, aoc24::day15::part2, "../data/day15/input.txt");
    bench_solution!(c, aoc24::day16::part1, "../data/day16/input.txt");
    bench_solution!(c, aoc24::day16::part2, "../data/day16/input.txt");
    bench_solution!(c, aoc24::day17::part1, "../data/day17/input.txt");
    bench_solution!(c, aoc24::day17::part2, "../data/day17/input.txt");
    bench_solution!(c, aoc24::day18::part1, "../data/day18/input.txt");
    bench_solution!(c, aoc24::day18::part2, "../data/day18/input.txt");
    bench_solution!(c, aoc24::day19::part1, "../data/day19/input.txt");
    bench_solution!(c, aoc24::day19::part2, "../data/day19/input.txt");
    bench_solution!(c, aoc24::day20::part1, "../data/day20/input.txt");
    bench_solution!(c, aoc24::day20::part2, "../data/day20/input.txt");
    bench_solution!(c, aoc24::day21::part1, "../data/day21/input.txt");
    bench_solution!(c, aoc24::day21::part2, "../data/day21/input.txt");
    bench_solution!(c, aoc24::day22::part1, "../data/day22/input.txt");
    bench_solution!(c, aoc24::day22::part2, "../data/day22/input.txt");
    bench_solution!(c, aoc24::day23::part1, "../data/day23/input.txt");
    bench_solution!(c, aoc24::day23::part2, "../data/day23/input.txt");
    bench_solution!(c, aoc24::day24::part1, "../data/day24/input.txt");
}
