#!/usr/bin/env python3
"""Sum the calibration values from the calibration document."""

import re
from pathlib import Path
from typing import Optional


regex_digits = re.compile(r"\d")


def get_calibration_value(text: str) -> Optional[int]:
    """Extract the calibration code from a line of text."""
    try:
        digits = regex_digits.findall(text)
        first_digit = digits[0]
        last_digit = digits[-1]
        return int(f"{first_digit}{last_digit}")
    except AttributeError:
        print(f"Line {text!r} does not match the regex.")
        return None


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
