#!/usr/bin/env python3
"""Sum the IDs of the possible games."""

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


@dataclass
class Game:
    id: int
    cube_sets: List[CubeSet]


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


def games_playable_with_cube_set(games: List[Game], cube_set: CubeSet) -> List[Game]:
    return [
        game
        for game in games
        if all(game_cube_set <= cube_set for game_cube_set in game.cube_sets)
    ]


def main(input_path: Path) -> int:
    """Sum the IDs of the possible games."""
    games = parse_game_data(input_path)
    print(f"Parsed {len(games)} games.")

    cube_set = CubeSet(red=12, green=13, blue=14)
    print(cube_set)

    games_playable = games_playable_with_cube_set(games, cube_set)
    print(f"Games playable: {len(games_playable)}")

    game_sum = sum(game.id for game in games_playable)
    print(f"Game Sum: {game_sum}")

    return game_sum


if __name__ == "__main__":
    main(here / "input.txt")
