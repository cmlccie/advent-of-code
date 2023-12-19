#!/usr/bin/env python3
"""Day 6: Wait For It - Part 1."""

from pathlib import Path
from dataclasses import dataclass
from typing import List, Iterator
from functools import reduce


here = Path(__file__).parent


@dataclass
class RaceData:
    time: int
    distance: int


@dataclass
class RaceResult:
    time: int
    hold_time: int
    speed: int
    distance: int


def get_input(path: Path) -> List[RaceData]:
    """Get input from file."""
    with open(path) as file:
        input_lines = list(file.readlines())

    times = [int(time) for time in input_lines[0].split(":")[1].split()]
    distances = [int(distance) for distance in input_lines[1].split(":")[1].split()]

    return [RaceData(time, distance) for time, distance in zip(times, distances)]


def generate_race_results(time: int) -> Iterator[RaceResult]:
    for hold_time in range(1, time):
        speed = hold_time
        yield RaceResult(time, hold_time, speed, distance=(time - hold_time) * speed)


def calculate_ways_to_win(race_data: List[RaceData]) -> List[RaceResult]:
    return [
        reduce(
            lambda total, _: total + 1,
            (
                race_result
                for race_result in generate_race_results(time=race.time)
                if race_result.distance > race.distance
            ),
            0,
        )
        for race in race_data
    ]


def main(path: Path) -> int:
    race_data = get_input(path)
    print(f"Parsed {len(race_data)} races from the input file.")

    ways_to_win = calculate_ways_to_win(race_data)
    print(f"Found {ways_to_win!r} ways to win.")

    product = reduce(lambda x, y: x * y, ways_to_win)
    print(f"Product of ways to win is {product!r}.")

    return product


if __name__ == "__main__":
    main(here / "input.txt")
