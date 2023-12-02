#!/usr/bin/env python3
"""Sum the power of the minimum cube set for each game."""

from dataclasses import dataclass
from typing import List
from pathlib import Path

here = Path(__file__).parent


@dataclass
class CubeSet:
    red: int = 0
    green: int = 0
    blue: int = 0

    def __le__(self, other: "CubeSet") -> bool:
        return all(
            (
                self.red <= other.red,
                self.green <= other.green,
                self.blue <= other.blue,
            )
        )

    def power(self) -> int:
        return self.red * self.green * self.blue


@dataclass
class Game:
    id: int
    cube_sets: List[CubeSet]

    def min_cube_set(self) -> CubeSet:
        min_cube_set = CubeSet()
        for cube_set in self.cube_sets:
            min_cube_set.red = max(min_cube_set.red, cube_set.red)
            min_cube_set.green = max(min_cube_set.green, cube_set.green)
            min_cube_set.blue = max(min_cube_set.blue, cube_set.blue)
        return min_cube_set


def parse_game_data(path: Path) -> List[Game]:
    with open(path) as file:
        return [
            Game(
                id=int(game_id),
                cube_sets=[
                    CubeSet(
                        **{
                            color: int(count)
                            for cube_count_str in cube_set_str.split(", ")
                            for count, color in [cube_count_str.split(" ")]
                        }
                    )
                    for cube_set_str in cube_sets_str.split("; ")
                ],
            )
            for line in file
            for game, cube_sets_str in [line.strip().split(": ")]
            for _, game_id in [game.split(" ")]
        ]


def main(input_path: Path) -> int:
    """Sum the power of the minimum cube set for each game."""
    games = parse_game_data(input_path)
    print(f"Parsed {len(games)} games.")

    min_cube_sets = [game.min_cube_set() for game in games]

    power_sum = sum(cube_set.power() for cube_set in min_cube_sets)
    print(f"Power Sum: {power_sum}")

    return power_sum


if __name__ == "__main__":
    main(here / "input.txt")
