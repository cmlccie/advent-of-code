"""Test Part 1."""

from pathlib import Path

import part1

tests_dir = Path(__file__).parent


# --------------------------------------------------------------------------------------
# Test Part 1
# --------------------------------------------------------------------------------------


def test_parse_cards():
    """Test parsing the cards."""
    cards = part1.parse_cards(tests_dir / "part1_example.txt")
    assert len(cards) == 6

    points = [card.points() for card in cards]
    assert points == [8, 2, 2, 1, 0, 0]


def test_part1_example():
    """Test the example from the puzzle text."""
    assert part1.main(tests_dir / "part1_example.txt") == 13


def test_part1_solution():
    """Test the example from the puzzle text."""
    assert part1.main() == 25183
