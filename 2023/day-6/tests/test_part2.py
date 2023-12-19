"""Day 6 - Part 2 tests."""

from pathlib import Path
import part2

tests_dir = Path(__file__).parent
code_dir = tests_dir.parent


def test_part2_example():
    answer = part2.main(tests_dir / "part1_example.txt")
    assert answer == 71503


def test_part2_solution():
    answer = part2.main(code_dir / "input.txt")
    assert answer == 34788142
