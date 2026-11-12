package aoc15;

import java.util.Optional;

/*-------------------------------------------------------------------------------------------------
  Day 1: Not Quite Lisp
-------------------------------------------------------------------------------------------------*/

public final class Day1 implements Solution {
    @Override
    public String title() {
        return "Day 1: Not Quite Lisp";
    }

    @Override
    public Optional<String> part1(String input) {
        int floor = 0;
        for (char c : input.toCharArray()) {
            switch (c) {
                case '(' -> floor++;
                case ')' -> floor--;
                case '\n' -> {} // ignore newlines
                default -> System.out.println("Other: " + c);
            }
        }
        return Optional.of(String.valueOf(floor));
    }

    @Override
    public Optional<String> part2(String input) {
        int floor = 0;
        for (int i = 0; i < input.length(); i++) {
            char c = input.charAt(i);
            switch (c) {
                case '(' -> floor++;
                case ')' -> floor--;
                case '\n' -> {} // ignore newlines
                default -> System.out.println("Other: " + c);
            }
            if (floor == -1) {
                return Optional.of(String.valueOf(i + 1));
            }
        }
        return Optional.empty();
    }

    /*--------------------------------------------------------------------------------------
      Core
    --------------------------------------------------------------------------------------*/

}
