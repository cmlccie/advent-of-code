"""Part 2 tests."""

from pathlib import Path

import part2
import pytest

tests_dir = Path(__file__).parent.resolve()
code_dir = tests_dir.parent


@pytest.fixture(scope="module")
def almanac() -> str:
    return part2.get_almanac(tests_dir / "part1_example.txt")


def test_part2_example():
    solution = part2.main(tests_dir / "part1_example.txt")
    assert solution == 46


def test_part1_solution():
    solution = part2.main(code_dir / "input.txt")
    assert solution == 63179500
