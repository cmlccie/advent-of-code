#!/usr/bin/env python3
"""Day 5: If You Give A Seed A Fertilizer - Part 2."""

import re
from pathlib import Path
from typing import List, Callable, Optional, Dict, NamedTuple, Tuple
from itertools import batched
from tqdm import tqdm

here = Path(__file__).parent.resolve()

# --------------------------------------------------------------------------------------
# Regex Patterns
# --------------------------------------------------------------------------------------


seeds_regex = re.compile(r"seeds: (?P<seeds>[0-9 ]+)")
maps_regex = re.compile(
    r"^(?P<source>[a-z]+)-to-(?P<destination>[a-z]+) map:\n(?P<maps>(?:\d+ \d+ \d+\n)+)",
    flags=re.MULTILINE,
)

# --------------------------------------------------------------------------------------
# Helper Classes
# --------------------------------------------------------------------------------------


class Item(NamedTuple):
    category: str
    number: int


Map = Callable[[Item], Optional[Item]]


# --------------------------------------------------------------------------------------
# Helper Functions
# --------------------------------------------------------------------------------------


def get_almanac(input: Path) -> str:
    with open(input) as file:
        return file.read()


def get_seed_ranges(almanac: str) -> List[Tuple[int, int]]:
    return [
        (start, start + range_length)
        for str_values in [seeds_regex.search(almanac).group("seeds").split()]
        for start, range_length in batched(map(int, str_values), 2)
    ]


def get_location_ranges(almanac: str) -> List[Tuple[int, int]]:
    locations_match = [
        match
        for match in maps_regex.finditer(almanac)
        if match.group("destination") == "location"
    ][0]

    location_ranges = sorted(
        [
            (destination_range_start, destination_range_start + range_length)
            for line in locations_match.group("maps").splitlines()
            for destination_range_start, _, range_length in [
                list(map(int, line.split()))
            ]
        ]
    )

    if location_ranges[0][0] != 0:
        location_ranges.insert(0, (0, location_ranges[0][0] - 1))

    return location_ranges


# --------------------------------------------------------------------------------------
# Part 2 Solution Functions
# --------------------------------------------------------------------------------------


def create_item_map(
    source_category: str,
    destination_category: str,
    destination_range_start: int,
    source_range_start: int,
    range_length: int,
) -> Map:
    def item_map(item: Item) -> Optional[Item]:
        if (item.category == destination_category) and (
            destination_range_start
            <= item.number
            < (destination_range_start + range_length)
        ):
            return Item(
                category=source_category,
                number=item.number - destination_range_start + source_range_start,
            )
        else:
            return None

    return item_map


def create_default_map(source_category: str, destination_category: str) -> Map:
    def default_map(item: Item) -> Optional[Item]:
        if item.category == destination_category:
            return Item(category=source_category, number=item.number)
        else:
            print(f"Item did not match default_map: {item!r}")
            return None

    return default_map


def build_reverse_maps(almanac: str) -> Dict[str, List[Map]]:
    return {
        destination: [
            create_item_map(
                source_category=source,
                destination_category=destination,
                destination_range_start=destination_range_start,
                source_range_start=source_range_start,
                range_length=range_length,
            )
            for line in maps.splitlines()
            for destination_range_start, source_range_start, range_length in [
                list(map(int, line.split()))
            ]
        ]
        + [create_default_map(source, destination)]
        for match in maps_regex.finditer(almanac)
        for source in [match.group("source")]
        for destination in [match.group("destination")]
        for maps in [match.group("maps")]
    }


def map_item(item: Item, maps: Dict[str, List[Map]]) -> Item:
    current_item = item
    while current_item.category in maps:
        for current_map in maps[current_item.category]:
            if mapped_item := current_map(current_item):
                current_item = mapped_item
                break
        else:
            raise ValueError(f"No map found for {current_item}")

    return current_item


def seed_validator(seed_ranges: List[Tuple[int, int]]) -> Callable[[Item], bool]:
    def validator(item: Item) -> bool:
        return item.category == "seed" and any(
            start <= item.number < end for start, end in seed_ranges
        )

    return validator


def find_minumum_seed(
    lower_bound: int,
    upper_bound: int,
    maps: Dict[str, List[Map]],
    valid_seed: Callable[[Item], bool],
) -> Item:
    """Uses binary search to find the minimum seed in the range."""
    test = (lower_bound + upper_bound) // 2  # midpoint
    one_before = test - 1

    test_location = Item(category="location", number=test)
    one_before_location = Item(category="location", number=one_before)

    test_seed = map_item(test_location, maps)
    one_before_seed = map_item(one_before_location, maps)

    test_location_is_valid_seed = valid_seed(test_seed)

    if test_location_is_valid_seed and not valid_seed(one_before_seed):
        return Item(category="location", number=test)

    if test_location_is_valid_seed:
        return find_minumum_seed(lower_bound, test, maps, valid_seed)
    else:
        return find_minumum_seed(test, upper_bound, maps, valid_seed)


def get_closest_location(
    location_ranges: List[Tuple[int, int]],
    maps: Dict[str, List[Map]],
    valid_seed: Callable[[Item], bool],
) -> Item:
    for location_start, location_end in location_ranges:
        start_location = Item(category="location", number=location_start)
        start_seed = map_item(start_location, maps)
        if valid_seed(start_seed):
            return start_location

        end_location = Item(category="location", number=location_end)
        end_seed = map_item(end_location, maps)
        if valid_seed(end_seed):
            print(
                f"Found valid seed at end of range: ({start_location}, {end_location})"
            )
            for location in tqdm(range(location_start, location_end)):
                location_item = Item(category="location", number=location)
                seed_item = map_item(location_item, maps)
                if valid_seed(seed_item):
                    return location_item


# --------------------------------------------------------------------------------------
# Main Function
# --------------------------------------------------------------------------------------


def main(input: Path) -> int:
    almanac = get_almanac(input)
    maps = build_reverse_maps(almanac)
    seed_ranges = get_seed_ranges(almanac)
    valid_seed = seed_validator(seed_ranges)

    location_ranges = get_location_ranges(almanac)

    closest_location = get_closest_location(location_ranges, maps, valid_seed)

    print(f"Closest location is {closest_location.number}.")
    return closest_location.number


if __name__ == "__main__":
    main(here / "input.txt")
