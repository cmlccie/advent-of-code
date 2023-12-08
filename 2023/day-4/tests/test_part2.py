"""Test Part 2."""

from pathlib import Path

import part2

tests_dir = Path(__file__).parent


def test_part2_example():
    """Test the example from the puzzle text."""
    assert part2.main(tests_dir / "part2_example.txt") == 30


def test_part2_solution():
    """Test the solution."""
    assert part2.main() == 5667240
