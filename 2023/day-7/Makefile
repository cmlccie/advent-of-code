.PHONY: run part1 part2 release clean

run: part1 part2

part1: .build/debug/part1
	.build/debug/part1

part2: .build/debug/part2
	.build/debug/part2

.build/debug/part1: Sources/part1/main.swift
	swift build --product part1

.build/debug/part2: Sources/part2/main.swift
	swift build --product part2

release: .build/release/part1 .build/release/part2

.build/release/part1: Sources/part1/main.swift
	swift build -c release --product part1

.build/release/part2: Sources/part2/main.swift
	swift build -c release --product part2

clean:
	rm -rf .build
