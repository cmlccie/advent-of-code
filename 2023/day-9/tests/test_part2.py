"""Test Part 2."""

from pathlib import Path
import part2


tests_dir = Path(__file__).parent
code_dir = tests_dir.parent


def test_predict_next_value():
    histories = part2.get_input(tests_dir / "part1_example.txt")
    predicted_previous_values = [
        part2.predict_previous_value(history) for history in histories
    ]
    expected_previous_values = [-3, 0, 5]
    assert predicted_previous_values == expected_previous_values


def test_main():
    answer = part2.main(tests_dir / "part1_example.txt")
    assert answer == 2


def test_part1_solution():
    answer = part2.main(code_dir / "input.txt")
    assert answer == 864
