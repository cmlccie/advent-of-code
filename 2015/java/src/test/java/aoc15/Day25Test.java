package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 25: Let It Snow
-------------------------------------------------------------------------------------------------*/

class Day25Test {
    private static final Solution DAY = new Day25();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day25/example.txt", "../data/day25/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day25/example.txt", "../data/day25/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day25/input.txt", "../data/day25/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day25/input.txt", "../data/day25/input-part2-answer.txt");
    }
}
