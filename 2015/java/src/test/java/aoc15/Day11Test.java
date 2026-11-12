package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 11: Corporate Policy
-------------------------------------------------------------------------------------------------*/

class Day11Test {
    private static final Solution DAY = new Day11();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day11/example.txt", "../data/day11/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day11/example.txt", "../data/day11/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day11/input.txt", "../data/day11/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day11/input.txt", "../data/day11/input-part2-answer.txt");
    }
}
