#!/usr/bin/env python3
"""Sum the calibration values from the calibration document."""

from pathlib import Path
from typing import Optional, Dict, Union


digits = {
    "1": "1",
    "2": "2",
    "3": "3",
    "4": "4",
    "5": "5",
    "6": "6",
    "7": "7",
    "8": "8",
    "9": "9",
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}

TrieNode = Dict[str, Union[str, "TrieNode"]]


def add(trie_node: TrieNode, chars: str, value: int) -> TrieNode:
    assert len(chars) > 0
    c = chars[0]
    if len(chars) == 1:
        # Base case - add the value to the trie node.
        trie_node[c] = value
    else:
        # Recursive case - add the next character to the trie node.
        next_node = trie_node.setdefault(c, {})
        add(next_node, chars[1:], value)


def trie_match(trie_node: TrieNode, text: str) -> Optional[str]:
    """Match the beginning of a string to a word in the trie."""
    if len(text) == 0:
        # Base case - empty string.
        return None

    next_node = trie_node.get(text[0])

    if next_node is None:
        # Base case - the beginning of the string is not in the trie.
        return None

    elif isinstance(next_node, str):
        # Base case - found a match! Return the value.
        return next_node

    else:
        # Recursive case - continue matching the string with the next trie node.
        return trie_match(next_node, text[1:])


class Trie:
    """Find digits using a trie search."""

    trie: TrieNode

    def __init__(self, digits_map: Dict[str, str], reversed: bool = False) -> None:
        """Initialize the trie."""
        self.trie = {}
        for word, digit in digits_map.items():
            if reversed:
                word = word[::-1]
            add(self.trie, word, digit)

    def search(self, text: str) -> Optional[str]:
        """Search for a value in the trie."""
        for i in range(len(text)):
            value = trie_match(self.trie, text[i:])
            if value is not None:
                return value

        return None


first_digit_trie = Trie(digits)
last_digit_trie = Trie(digits, reversed=True)


def get_calibration_value(text: str) -> Optional[int]:
    """Extract the calibration value from a line of text."""
    first_digit = first_digit_trie.search(text)
    last_digit = last_digit_trie.search(text[::-1])

    if first_digit is None:
        print(f"First digit not found in: {text!r}")
        return None

    if last_digit is None:
        print(f"Last digit not found in: {text!r}")
        return None

    calibration_value = int(f"{first_digit}{last_digit}")
    print(f"{calibration_value} <- {text!r}")
    return calibration_value


def main() -> int:
    """Sum the calibration values from the calibration document."""
    here = Path(__file__).parent
    with open(here / "calibration-document.txt") as file:
        calibration_values = [
            calibration_value
            for line in file
            for calibration_value in [get_calibration_value(line)]
            if calibration_value is not None
        ]

    sum_calibration_values = sum(calibration_values)

    print(f"Extracted {len(calibration_values)} calibration values.")
    print(f"Sum of calibration Values: {sum_calibration_values}")

    return sum_calibration_values


if __name__ == "__main__":
    main()
