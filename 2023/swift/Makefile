.PHONY: clean format lint tests build release

clean:
	swift package clean
	rm -rf .build

format:
	swift-format format --in-place --recursive Package.swift Sources/ Tests/

lint:
	swift-format lint --strict --recursive Package.swift Sources/ Tests/

tests:
	swift test

build:
	swift build

release:
	swift build -c release
