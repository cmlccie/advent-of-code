# -------------------------------------------------------------------------------------------------
# Advent of Code
# -------------------------------------------------------------------------------------------------

# --------------------------------------------------------------------------------------
# Current Year Targets
# --------------------------------------------------------------------------------------

.DEFAULT_GOAL := build
.PHONY: tests tests-slow build

tests: tests-2015

tests-slow: tests-slow-2015

test-benches: tests-benches-2015

build: build-2015

bench: bench-2015

# --------------------------------------------------------------------------------------
# Repository Targets
# --------------------------------------------------------------------------------------

.PHONY: unlock

make unlock:
	git-crypt unlock


# --------------------------------------------------------------------------------------
# 2024 Targets
# --------------------------------------------------------------------------------------

.PHONY: tests-2024 tests-slow-2024 build-2024

AOC_2024_SOURCE_FILES := $(shell find 2024/rust -type f)

tests-2024:
	cargo test --package aoc24

tests-slow-2024:
	cargo test --package aoc24 --features slow_tests

tests-benches-2024:
	cargo test --package aoc24 --benches

build-2024: 2024/aoc

2024/aoc: target/release/aoc24
	cd 2024 && ln -sf ../target/release/aoc24 aoc

target/release/aoc24: $(AOC_2024_SOURCE_FILES)
	cargo build --package aoc24 --release

bench-2024:
	cargo bench --package aoc24


# --------------------------------------------------------------------------------------
# 2015 Targets
# --------------------------------------------------------------------------------------

.PHONY: tests-2015 tests-slow-2015 build-2015 clean-2015 data-2015

AOC_2015_GRADLE := cd 2015/java && ./gradlew --quiet

tests-2015:
	$(AOC_2015_GRADLE) test

tests-slow-2015:
	$(AOC_2015_GRADLE) testSlow

build-2015:
	$(AOC_2015_GRADLE) installDist
	cd 2015 && ln -sf java/build/install/aoc15/bin/aoc15 aoc

clean-2015:
	$(AOC_2015_GRADLE) clean
	rm -f 2015/aoc

# Create empty data files for a day: make data-2015 DAY=1
data-2015:
	@test -n "$(DAY)" || (echo "usage: make data-2015 DAY=<n>" && exit 1)
	mkdir -p 2015/data/day$(DAY)
	cd 2015/data/day$(DAY) && touch example.txt example-part1-answer.txt example-part2-answer.txt \
		input.txt input-part1-answer.txt input-part2-answer.txt
