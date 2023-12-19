"""Day 6 - Part 1 tests."""

from pathlib import Path
import part1

tests_dir = Path(__file__).parent
code_dir = tests_dir.parent


def test_part1_example():
    answer = part1.main(tests_dir / "part1_example.txt")
    assert answer == 288


def test_part1_solution():
    answer = part1.main(code_dir / "input.txt")
    assert answer == 2756160
