#!/usr/bin/env python3
"""Day 9: Mirage Maintenance - Part 1."""

from pathlib import Path
from typing import List

here = Path(__file__).parent


def get_input(path: Path) -> List[List[int]]:
    with open(path) as file:
        return [[int(number) for number in line.split()] for line in file]


def predict_next_value(history: List[int]) -> int:
    layers: List[List[int]] = [history]

    while not all((value == 0 for value in layers[-1])):
        previous_layer = layers[-1]
        assert len(previous_layer) > 1
        differences = [
            previous_layer[i] - previous_layer[i - 1]
            for i in range(1, len(previous_layer))
        ]
        layers.append(differences)

    assert len(layers) > 1
    layers[-1].append(0)
    for i in range(len(layers) - 1, 0, -1):
        next_value = layers[i - 1][-1] + layers[i][-1]
        layers[i - 1].append(next_value)

    return next_value


def main(input_path: Path) -> int:
    histories = get_input(input_path)
    next_values = [predict_next_value(history) for history in histories]

    answer = sum(next_values)
    print(f"Part 1 - Sum of Predicted Next Values: {answer}")

    return answer


if __name__ == "__main__":
    main(here / "input.txt")
