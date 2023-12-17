"""Part 1 tests."""

from pathlib import Path

import part1
import pytest

tests_dir = Path(__file__).parent.resolve()
code_dir = tests_dir.parent


@pytest.fixture(scope="module")
def almanac() -> str:
    return part1.get_almanac(tests_dir / "part1_example.txt")


def test_almanac(almanac):
    assert isinstance(almanac, str)
    assert len(almanac) > 0


def test_get_seeds(almanac):
    seeds = part1.get_seeds(almanac)
    assert isinstance(seeds, list)
    assert seeds == [("seed", 79), ("seed", 14), ("seed", 55), ("seed", 13)]


def test_example_data(almanac):
    seeds = part1.get_seeds(almanac)
    maps = part1.build_maps(almanac)
    locations = [part1.map_item(seed, maps) for seed in seeds]
    assert locations == [
        ("location", 82),
        ("location", 43),
        ("location", 86),
        ("location", 35),
    ]


def test_part1_solution():
    solution = part1.main(code_dir / "input.txt")
    assert solution == 265018614
