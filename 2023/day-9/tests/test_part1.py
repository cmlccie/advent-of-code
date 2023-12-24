"""Test Part 1."""

from pathlib import Path
import part1


tests_dir = Path(__file__).parent
code_dir = tests_dir.parent


def test_predict_next_value():
    histories = part1.get_input(tests_dir / "part1_example.txt")
    predicted_next_values = [part1.predict_next_value(history) for history in histories]
    expected_next_values = [18, 28, 68]
    assert predicted_next_values == expected_next_values


def test_main():
    answer = part1.main(tests_dir / "part1_example.txt")
    assert answer == 114


def test_part1_solution():
    answer = part1.main(code_dir / "input.txt")
    assert answer == 1647269739
