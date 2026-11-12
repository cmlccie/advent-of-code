package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 20: Infinite Elves and Infinite Houses
-------------------------------------------------------------------------------------------------*/

class Day20Test {
    private static final Solution DAY = new Day20();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day20/example.txt", "../data/day20/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day20/example.txt", "../data/day20/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day20/input.txt", "../data/day20/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day20/input.txt", "../data/day20/input-part2-answer.txt");
    }
}
