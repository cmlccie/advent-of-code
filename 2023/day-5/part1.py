#!/usr/bin/env python3
"""Day 5: If You Give A Seed A Fertilizer - Part 1."""

import re
from pathlib import Path
from typing import List, Callable, Optional, Dict, NamedTuple

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


def get_seeds(almanac: str) -> List[Item]:
    return [
        Item(category="seed", number=int(seed))
        for seed in seeds_regex.search(almanac).group("seeds").split()
    ]


# --------------------------------------------------------------------------------------
# Solution Functions
# --------------------------------------------------------------------------------------


def create_item_map(
    source_category: str,
    destination_category: str,
    destination_range_start: int,
    source_range_start: int,
    range_length: int,
) -> Map:
    def item_map(item: Item) -> Optional[Item]:
        if (item.category == source_category) and (
            source_range_start <= item.number < (source_range_start + range_length)
        ):
            return Item(
                category=destination_category,
                number=item.number - source_range_start + destination_range_start,
            )
        else:
            return None

    return item_map


def create_default_map(source_category: str, destination_category: str) -> Map:
    def default_map(item: Item) -> Optional[Item]:
        if item.category == source_category:
            return Item(category=destination_category, number=item.number)
        else:
            print(f"Item did not match default_map: {item!r}")
            return None

    return default_map


def build_maps(almanac: str) -> Dict[str, List[Map]]:
    return {
        source: [
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


def main(input: Path) -> int:
    almanac = get_almanac(input)
    seeds = get_seeds(almanac)
    maps = build_maps(almanac)
    locations = [map_item(seed, maps) for seed in seeds]
    closest_location = min(locations, key=lambda item: item.number)

    print(f"Closest location is {closest_location.number}.")
    return closest_location.number


if __name__ == "__main__":
    main(here / "input.txt")
