package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 18: Like a GIF For Your Yard
-------------------------------------------------------------------------------------------------*/

class Day18Test {
    private static final Solution DAY = new Day18();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day18/example.txt", "../data/day18/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day18/example.txt", "../data/day18/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day18/input.txt", "../data/day18/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day18/input.txt", "../data/day18/input-part2-answer.txt");
    }
}
