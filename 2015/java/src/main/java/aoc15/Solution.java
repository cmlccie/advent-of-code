package aoc15;

import java.util.Optional;

/*-------------------------------------------------------------------------------------------------
  Solution
-------------------------------------------------------------------------------------------------*/

/** A day's puzzle solution. Each part takes the raw puzzle input and returns its answer. */
public interface Solution {
    String title();

    Optional<String> part1(String input);

    Optional<String> part2(String input);
}
