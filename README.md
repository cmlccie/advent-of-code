# Advent of Code

2024 Complete! 😎 Now I'm going back and refactoring, benchmarking, and performance tuning - just for fun and learning! 🤓

You can find my **2024 Rust Solutions** in the [`2024/rust/src`](2024/rust/src) directory.

## Current Benchmarks

- Solution function times
- Excludes disk and terminal I/O and CLI setup code
- Includes input parsing

```text
aoc24::day1::part1      time:   [66.875 µs 66.934 µs 67.001 µs]
aoc24::day1::part2      time:   [104.58 µs 104.74 µs 104.91 µs]
aoc24::day2::part1      time:   [151.48 µs 151.58 µs 151.71 µs]
aoc24::day2::part2      time:   [258.48 µs 258.71 µs 258.94 µs]
aoc24::day3::part1      time:   [101.98 µs 102.16 µs 102.32 µs]
aoc24::day3::part2      time:   [62.152 µs 62.286 µs 62.427 µs]
aoc24::day4::part1      time:   [1.3000 ms 1.3035 ms 1.3069 ms]
aoc24::day4::part2      time:   [440.58 µs 441.62 µs 442.72 µs]
aoc24::day5::part1      time:   [388.60 µs 390.38 µs 392.31 µs]
aoc24::day5::part2      time:   [390.91 µs 393.05 µs 395.42 µs]
aoc24::day6::part1      time:   [698.73 µs 700.65 µs 702.53 µs]
aoc24::day6::part2      time:   [1.7814 s 1.7844 s 1.7874 s]
aoc24::day7::part1      time:   [3.5106 ms 3.5187 ms 3.5272 ms]
aoc24::day7::part2      time:   [143.65 ms 143.86 ms 144.08 ms]
aoc24::day8::part1      time:   [30.775 µs 30.833 µs 30.892 µs]
aoc24::day8::part2      time:   [131.21 µs 131.46 µs 131.70 µs]
aoc24::day9::part1      time:   [12.544 ms 12.576 ms 12.608 ms]
aoc24::day9::part2      time:   [2.1453 s 2.1466 s 2.1481 s]
aoc24::day10::part1     time:   [724.24 µs 725.51 µs 726.80 µs]
aoc24::day10::part2     time:   [721.60 µs 723.23 µs 724.77 µs]
aoc24::day11::part1     time:   [433.40 µs 433.87 µs 434.41 µs]
aoc24::day11::part2     time:   [17.568 ms 17.609 ms 17.655 ms]
aoc24::day12::part1     time:   [6.0624 ms 6.0667 ms 6.0714 ms]
aoc24::day12::part2     time:   [26.694 ms 26.741 ms 26.786 ms]
aoc24::day13::part1     time:   [464.98 µs 466.74 µs 468.59 µs]
aoc24::day13::part2     time:   [466.45 µs 467.84 µs 469.36 µs]
aoc24::day14::part1     time:   [634.00 µs 637.86 µs 641.79 µs]
aoc24::day14::part2     time:   [21.664 ms 21.697 ms 21.736 ms]
aoc24::day15::part1     time:   [1.3136 ms 1.3168 ms 1.3200 ms]
aoc24::day15::part2     time:   [1.5676 ms 1.5712 ms 1.5748 ms]
aoc24::day16::part1     time:   [87.959 ms 88.050 ms 88.151 ms]
aoc24::day16::part2     time:   [88.035 ms 88.185 ms 88.357 ms]
aoc24::day17::part1     time:   [187.35 µs 187.63 µs 187.93 µs]
aoc24::day17::part2     time:   [323.22 µs 327.58 µs 331.81 µs]
aoc24::day18::part1     time:   [1.1913 ms 1.1923 ms 1.1933 ms]
aoc24::day18::part2     time:   [1.2559 s 1.2572 s 1.2586 s]
aoc24::day19::part1     time:   [2.3693 ms 2.3718 ms 2.3744 ms]
aoc24::day19::part2     time:   [8.5330 ms 8.5833 ms 8.6325 ms]
aoc24::day20::part1     time:   [3.5215 ms 3.5258 ms 3.5312 ms]
aoc24::day20::part2     time:   [182.45 ms 182.68 ms 182.89 ms]
aoc24::day21::part1     time:   [10.310 µs 10.334 µs 10.359 µs]
aoc24::day21::part2     time:   [69.489 µs 69.539 µs 69.594 µs]
aoc24::day22::part1     time:   [8.0927 ms 8.1080 ms 8.1255 ms]
aoc24::day22::part2     time:   [226.27 ms 226.87 ms 227.48 ms]
aoc24::day23::part1     time:   [7.9279 ms 7.9374 ms 7.9479 ms]
aoc24::day23::part2     time:   [942.39 ms 945.06 ms 948.03 ms]
aoc24::day24::part1     time:   [213.62 µs 214.09 µs 214.54 µs]
aoc24::day24::part2     time:   [83.580 µs 83.817 µs 84.078 µs]
aoc24::day25::part1     time:   [226.36 µs 226.73 µs 227.06 µs]
```
