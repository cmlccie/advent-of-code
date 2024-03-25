.PHONY: all swift_aoc
.DEFAULT_GOAL := all

clean:
	$(MAKE) -C swift/ clean
	rm -f swift_aoc

all: swift_aoc

swift_aoc:
	$(MAKE) -C swift/ release
	ln -sf ./swift/.build/release/swift_aoc swift_aoc
