package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 8: Matchsticks
-------------------------------------------------------------------------------------------------*/

class Day8Test {
    private static final Solution DAY = new Day8();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day8/example.txt", "../data/day8/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day8/example.txt", "../data/day8/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day8/input.txt", "../data/day8/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day8/input.txt", "../data/day8/input-part2-answer.txt");
    }
}
