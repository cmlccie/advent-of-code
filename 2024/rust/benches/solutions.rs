use aoc24::shared::inputs::get_input;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    /*-------------------------------------------------------------------------
      Day 1
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day1/input.txt");

        c.bench_function("aoc24::day1::part1", |b| {
            b.iter(|| aoc24::day1::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day1::part2", |b| {
            b.iter(|| aoc24::day1::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 2
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day2/input.txt");

        c.bench_function("aoc24::day2::part1", |b| {
            b.iter(|| aoc24::day2::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day2::part2", |b| {
            b.iter(|| aoc24::day2::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 3
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day3/input.txt");

        c.bench_function("aoc24::day3::part1", |b| {
            b.iter(|| aoc24::day3::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day3::part2", |b| {
            b.iter(|| aoc24::day3::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 4
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day4/input.txt");

        c.bench_function("aoc24::day4::part1", |b| {
            b.iter(|| aoc24::day4::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day4::part2", |b| {
            b.iter(|| aoc24::day4::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 5
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day5/input.txt");

        c.bench_function("aoc24::day5::part1", |b| {
            b.iter(|| aoc24::day5::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day5::part2", |b| {
            b.iter(|| aoc24::day5::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 6
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day6/input.txt");

        c.bench_function("aoc24::day6::part1", |b| {
            b.iter(|| aoc24::day6::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day6::part2", |b| {
            b.iter(|| aoc24::day6::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 7
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day7/input.txt");

        c.bench_function("aoc24::day7::part1", |b| {
            b.iter(|| aoc24::day7::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day7::part2", |b| {
            b.iter(|| aoc24::day7::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 8
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day8/input.txt");

        c.bench_function("aoc24::day8::part1", |b| {
            b.iter(|| aoc24::day8::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day8::part2", |b| {
            b.iter(|| aoc24::day8::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 9
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day9/input.txt");

        c.bench_function("aoc24::day9::part1", |b| {
            b.iter(|| aoc24::day9::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day9::part2", |b| {
            b.iter(|| aoc24::day9::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 10
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day10/input.txt");

        c.bench_function("aoc24::day10::part1", |b| {
            b.iter(|| aoc24::day10::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day10::part2", |b| {
            b.iter(|| aoc24::day10::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 11
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day11/input.txt");

        c.bench_function("aoc24::day11::part1", |b| {
            b.iter(|| aoc24::day11::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day11::part2", |b| {
            b.iter(|| aoc24::day11::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 12
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day12/input.txt");

        c.bench_function("aoc24::day12::part1", |b| {
            b.iter(|| aoc24::day12::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day12::part2", |b| {
            b.iter(|| aoc24::day12::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 13
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day13/input.txt");

        c.bench_function("aoc24::day13::part1", |b| {
            b.iter(|| aoc24::day13::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day13::part2", |b| {
            b.iter(|| aoc24::day13::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 14
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day14/input.txt");

        c.bench_function("aoc24::day14::part1", |b| {
            b.iter(|| aoc24::day14::part1(black_box(&input), 101, 103));
        });

        c.bench_function("aoc24::day14::part2", |b| {
            b.iter(|| aoc24::day14::part2(black_box(&input), false));
        });
    }

    /*-------------------------------------------------------------------------
      Day 15
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day15/input.txt");

        c.bench_function("aoc24::day15::part1", |b| {
            b.iter(|| aoc24::day15::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day15::part2", |b| {
            b.iter(|| aoc24::day15::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 16
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day16/input.txt");

        c.bench_function("aoc24::day16::part1", |b| {
            b.iter(|| aoc24::day16::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day16::part2", |b| {
            b.iter(|| aoc24::day16::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 17
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day17/input.txt");

        c.bench_function("aoc24::day17::part1", |b| {
            b.iter(|| aoc24::day17::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day17::part2", |b| {
            b.iter(|| aoc24::day17::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 18
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day18/input.txt");

        c.bench_function("aoc24::day18::part1", |b| {
            b.iter(|| aoc24::day18::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day18::part2", |b| {
            b.iter(|| aoc24::day18::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 19
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day19/input.txt");

        c.bench_function("aoc24::day19::part1", |b| {
            b.iter(|| aoc24::day19::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day19::part2", |b| {
            b.iter(|| aoc24::day19::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 20
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day20/input.txt");

        c.bench_function("aoc24::day20::part1", |b| {
            b.iter(|| aoc24::day20::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day20::part2", |b| {
            b.iter(|| aoc24::day20::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 21
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day21/input.txt");

        c.bench_function("aoc24::day21::part1", |b| {
            b.iter(|| aoc24::day21::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day21::part2", |b| {
            b.iter(|| aoc24::day21::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 22
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day22/input.txt");

        c.bench_function("aoc24::day22::part1", |b| {
            b.iter(|| aoc24::day22::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day22::part2", |b| {
            b.iter(|| aoc24::day22::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 23
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day23/input.txt");

        c.bench_function("aoc24::day23::part1", |b| {
            b.iter(|| aoc24::day23::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day23::part2", |b| {
            b.iter(|| aoc24::day23::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 24
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day24/input.txt");

        c.bench_function("aoc24::day24::part1", |b| {
            b.iter(|| aoc24::day24::part1(black_box(&input)));
        });

        c.bench_function("aoc24::day24::part2", |b| {
            b.iter(|| aoc24::day24::part2(black_box(&input)));
        });
    }

    /*-------------------------------------------------------------------------
      Day 25
    -------------------------------------------------------------------------*/

    {
        let input = get_input("../data/day25/input.txt");

        c.bench_function("aoc24::day25::part1", |b| {
            b.iter(|| aoc24::day25::part1(black_box(&input)));
        });
    }
}
