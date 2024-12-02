# --------------------------------------------------------------------------------------
# 2024
# --------------------------------------------------------------------------------------

AOC_2024_SOURCE_FILES := $(shell find 2024/rust -type f)

2024/aoc: target/release/aoc24
	cd 2024 && ln -sf ../target/release/aoc24 aoc

target/release/aoc24: $(AOC_2024_SOURCE_FILES)
	cargo build -p aoc24 --release
