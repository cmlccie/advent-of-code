package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 14: Reindeer Olympics
-------------------------------------------------------------------------------------------------*/

class Day14Test {
    private static final Solution DAY = new Day14();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day14/example.txt", "../data/day14/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day14/example.txt", "../data/day14/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day14/input.txt", "../data/day14/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day14/input.txt", "../data/day14/input-part2-answer.txt");
    }
}
