# Advent of Code

2024 Complete! 😎 Now I'm going back and refactoring, benchmarking, and performance tuning - just for fun and learning! 🤓

You can find my **2024 Rust Solutions** in the [`2024/rust/src`](2024/rust/src) directory.

## Current Benchmarks

- Solution function times
- Excludes disk and terminal I/O and CLI setup code
- Includes input parsing

```text
aoc24::day1::part1      time:   [66.506 µs 66.597 µs 66.691 µs]
aoc24::day1::part2      time:   [104.49 µs 104.69 µs 104.93 µs]
aoc24::day2::part1      time:   [143.36 µs 143.78 µs 144.18 µs]
aoc24::day2::part2      time:   [265.50 µs 266.37 µs 267.33 µs]
aoc24::day3::part1      time:   [101.79 µs 101.85 µs 101.91 µs]
aoc24::day3::part2      time:   [63.286 µs 63.442 µs 63.616 µs]
aoc24::day4::part1      time:   [1.3522 ms 1.3547 ms 1.3575 ms]
aoc24::day4::part2      time:   [609.84 µs 613.73 µs 619.59 µs]
aoc24::day5::part1      time:   [372.03 µs 373.02 µs 374.22 µs]
aoc24::day5::part2      time:   [370.96 µs 371.27 µs 371.64 µs]
aoc24::day6::part1      time:   [689.10 µs 690.61 µs 692.09 µs]
aoc24::day6::part2      time:   [1.7902 s 1.7921 s 1.7945 s]
aoc24::day7::part1      time:   [3.5045 ms 3.5068 ms 3.5094 ms]
aoc24::day7::part2      time:   [145.54 ms 145.85 ms 146.15 ms]
aoc24::day8::part1      time:   [30.502 µs 30.532 µs 30.567 µs]
aoc24::day8::part2      time:   [139.52 µs 139.68 µs 139.84 µs]
aoc24::day9::part1      time:   [13.617 ms 13.627 ms 13.638 ms]
aoc24::day9::part2      time:   [2.1277 s 2.1287 s 2.1299 s]
aoc24::day10::part1     time:   [1.0429 ms 1.0461 ms 1.0501 ms]
aoc24::day10::part2     time:   [1.0408 ms 1.0468 ms 1.0526 ms]
aoc24::day11::part1     time:   [552.10 µs 553.11 µs 554.25 µs]
aoc24::day11::part2     time:   [19.578 ms 19.634 ms 19.700 ms]
aoc24::day12::part1     time:   [5.2703 ms 5.2840 ms 5.2990 ms]
aoc24::day12::part2     time:   [24.977 ms 25.115 ms 25.276 ms]
aoc24::day13::part1     time:   [527.56 µs 528.61 µs 529.90 µs]
aoc24::day13::part2     time:   [523.43 µs 524.47 µs 525.49 µs]
aoc24::day14::part1     time:   [673.56 µs 676.24 µs 679.30 µs]
aoc24::day14::part2     time:   [21.506 ms 21.525 ms 21.546 ms]
aoc24::day15::part1     time:   [3.5828 ms 3.5869 ms 3.5909 ms]
aoc24::day15::part2     time:   [3.8955 ms 3.9116 ms 3.9274 ms]
aoc24::day16::part1     time:   [89.354 ms 89.435 ms 89.530 ms]
aoc24::day16::part2     time:   [89.395 ms 89.465 ms 89.544 ms]
aoc24::day17::part1     time:   [233.56 µs 236.74 µs 239.58 µs]
aoc24::day17::part2     time:   [392.64 µs 393.02 µs 393.43 µs]
aoc24::day18::part1     time:   [1.0963 ms 1.0991 ms 1.1020 ms]
aoc24::day18::part2     time:   [1.0832 s 1.0847 s 1.0863 s]
aoc24::day19::part1     time:   [2.7635 ms 2.7675 ms 2.7724 ms]
aoc24::day19::part2     time:   [21.637 ms 21.762 ms 21.903 ms]
aoc24::day20::part1     time:   [3.7969 ms 3.8047 ms 3.8116 ms]
aoc24::day20::part2     time:   [158.10 ms 158.35 ms 158.60 ms]
aoc24::day21::part1     time:   [12.779 µs 12.800 µs 12.821 µs]
aoc24::day21::part2     time:   [75.516 µs 75.705 µs 75.881 µs]
aoc24::day22::part1     time:   [8.0531 ms 8.0571 ms 8.0620 ms]
aoc24::day22::part2     time:   [226.49 ms 226.88 ms 227.24 ms]
aoc24::day23::part1     time:   [7.9710 ms 7.9778 ms 7.9853 ms]
aoc24::day23::part2     time:   [1.0675 s 1.0748 s 1.0835 s]
aoc24::day24::part1     time:   [240.61 µs 240.90 µs 241.18 µs]
aoc24::day24::part2     time:   [117.44 µs 117.52 µs 117.62 µs]
aoc24::day25::part1     time:   [231.91 µs 232.23 µs 232.56 µs]
```
