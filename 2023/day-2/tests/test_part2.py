import part2

from pathlib import Path

here = Path(__file__).parent
code_dir = here.parent


def test_part2_example_power_sum():
    assert part2.main(here / "part2_examples.txt") == 2286


def test_part2():
    assert part2.main(code_dir / "input.txt") == 78375
