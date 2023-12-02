import part1

from pathlib import Path

here = Path(__file__).parent
code_dir = here.parent


def test_part1_examples():
    assert part1.main(here / "part1_examples.txt") == 8


def test_part1():
    assert part1.main(code_dir / "input.txt") == 2406
