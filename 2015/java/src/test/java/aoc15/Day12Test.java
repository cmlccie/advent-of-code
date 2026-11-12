package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 12: JSAbacusFramework.io
-------------------------------------------------------------------------------------------------*/

class Day12Test {
    private static final Solution DAY = new Day12();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day12/example.txt", "../data/day12/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day12/example.txt", "../data/day12/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day12/input.txt", "../data/day12/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day12/input.txt", "../data/day12/input-part2-answer.txt");
    }
}
