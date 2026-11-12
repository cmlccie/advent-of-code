package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 13: Knights of the Dinner Table
-------------------------------------------------------------------------------------------------*/

class Day13Test {
    private static final Solution DAY = new Day13();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day13/example.txt", "../data/day13/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day13/example.txt", "../data/day13/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day13/input.txt", "../data/day13/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day13/input.txt", "../data/day13/input-part2-answer.txt");
    }
}
