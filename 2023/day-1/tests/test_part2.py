"""Tests for Day-1 Part 2."""

import solution_part2


def test_part2_examples():
    test_data = {
        "two1nine": 29,
        "eightwothree": 83,
        "abcone2threexyz": 13,
        "xtwone3four": 24,
        "4nineeightseven2": 42,
        "zoneight234": 14,
        "7pqrstsixteen": 76,
    }

    for line, expected_value in test_data.items():
        calibration_value = solution_part2.get_calibration_value(line)
        assert calibration_value == expected_value


def test_part2_eightwo():
    test_data = {
        "ceightwoninelkbbfxgsv9fb5n": 85,
        "rveightwo79three": 83,
        "threednfntx4eightwovql": 32,
        "175rpdmxfeightwos": 12,
        "6foureightwofh": 62,
        "flcpl3btfmbbpnkjvnlmcthreetwo1eightwops": 32,
        "gteightwoone268four": 84,
        "jeightwo47three86twoseven": 87,
        "rkeightwo6zfpvrfgqr7qxbkkg43lrjqtzjrprqttxmbrzg": 83,
        "tkgrnhbflp7zltmbdoneeightwoh": 72,
        "leightwo5": 85,
        "8fiveeightonetwovgvhzgzfjh16eightwohlk": 82,
        "lnveightwohdkgcvvrjs38": 88,
        "qkeightwotwocjcngknkztwo7": 87,
        "three2dpsdhfld95eightwoht": 32,
        "qfeightwo9threethree": 83,
        "twofive4eightwozz": 22,
        "threefnhxtdbl1jtxeightwol": 32,
        "3nineeightwokh": 32,
    }

    for line, expected_value in test_data.items():
        calibration_value = solution_part2.get_calibration_value(line)
        assert calibration_value == expected_value


def test_main():
    assert solution_part2.main() == 53866
