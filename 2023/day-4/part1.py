#!/usr/bin/env python3
"""Scratchcards."""

from dataclasses import dataclass
from pathlib import Path
from typing import List, Set

code_dir = Path(__file__).parent

# --------------------------------------------------------------------------------------
# Scratchcard
# --------------------------------------------------------------------------------------


@dataclass
class Scratchcard:
    """A scratchcard."""

    number: int
    winning_numbers: Set[int]
    your_numbers: Set[int]

    @classmethod
    def from_line(cls, line: str) -> "Scratchcard":
        """Parse a Card from a line from the input file."""
        card_identifier, numbers = line.split(": ")
        number = card_identifier.split()[1]
        winning_numbers, your_numbers = numbers.split(" | ")
        return cls(
            number=int(number),
            winning_numbers=set(int(x) for x in winning_numbers.split()),
            your_numbers=set(int(x) for x in your_numbers.split()),
        )

    def my_winning_numbers(self) -> Set[int]:
        """Return the winning numbers on the card."""
        return self.winning_numbers.intersection(self.your_numbers)

    def points(self) -> int:
        """Calculate the card's point value."""
        matches = len(self.my_winning_numbers())
        if matches == 0:
            return 0
        else:
            return 2 ** (matches - 1)


def parse_cards(path: Path) -> List[Scratchcard]:
    """Parse the cards from the input file."""
    with open(path) as file_handle:
        cards = [Scratchcard.from_line(line) for line in file_handle]
        print(f"Parsed {len(cards)} cards.")
    return cards


def main(path: Path = code_dir / "input.txt") -> int:
    """Calculate the total point value of the elf's scratchcards."""
    cards = parse_cards(path)

    total = sum(card.points() for card in cards)
    print(f"Total points: {total}")

    return total


if __name__ == "__main__":
    main()
