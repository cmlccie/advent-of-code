"""Tests for Day-1 Part 1."""

import solution_part1


def test_part2_examples():
    test_data = {
        "1abc2": 12,
        "pqr3stu8vwx": 38,
        "a1b2c3d4e5f": 15,
        "treb7uchet": 77,
    }

    for line, expected_value in test_data.items():
        calibration_value = solution_part1.get_calibration_value(line)
        assert calibration_value == expected_value


def test_main():
    assert solution_part1.main() == 54159
