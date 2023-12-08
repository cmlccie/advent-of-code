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

    def matches(self) -> int:
        return len(self.my_winning_numbers())


def parse_cards(path: Path) -> List[Scratchcard]:
    """Parse the cards from the input file."""
    with open(path) as file_handle:
        cards = [Scratchcard.from_line(line) for line in file_handle]
        print(f"Parsed {len(cards)} cards.")
    return cards


def main(path: Path = code_dir / "input.txt") -> int:
    """Calculate how many scratch cards we win."""
    original_cards_by_number = {card.number: card for card in parse_cards(path)}
    cards_won_by_number = {
        original_number: [
            copy
            for copy_offset in range(1, original_card.matches() + 1)
            for index in [original_number + copy_offset]
            for copy in [original_cards_by_number.get(index)]
            if copy is not None
        ]
        for original_number, original_card in original_cards_by_number.items()
    }

    cards_won = list(original_cards_by_number.values())
    index = 0
    while index < len(cards_won):
        current_card = cards_won[index]
        cards_won += cards_won_by_number[current_card.number]
        index += 1

    count_cards_won = len(cards_won)
    print(f"You won {count_cards_won} cards.")

    return count_cards_won


if __name__ == "__main__":
    main()
