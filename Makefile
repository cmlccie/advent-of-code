# -------------------------------------------------------------------------------------------------
# Advent of Code
# -------------------------------------------------------------------------------------------------

# --------------------------------------------------------------------------------------
# Current Year Targets
# --------------------------------------------------------------------------------------

.DEFAULT_GOAL := build
.PHONY: tests build

tests: tests-2024

build: build-2024

# --------------------------------------------------------------------------------------
# Repository Targets
# --------------------------------------------------------------------------------------

.PHONY: unlock

make unlock:
	git-crypt unlock


# --------------------------------------------------------------------------------------
# 2024 Targets
# --------------------------------------------------------------------------------------

.PHONY: tests-2024 build-2024

AOC_2024_SOURCE_FILES := $(shell find 2024/rust -type f)

tests-2024:
	cargo test --package aoc24

build-2024: 2024/aoc

2024/aoc: target/release/aoc24
	cd 2024 && ln -sf ../target/release/aoc24 aoc

target/release/aoc24: $(AOC_2024_SOURCE_FILES)
	cargo build --package aoc24 --release
