package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 23: Opening the Turing Lock
-------------------------------------------------------------------------------------------------*/

class Day23Test {
    private static final Solution DAY = new Day23();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day23/example.txt", "../data/day23/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day23/example.txt", "../data/day23/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day23/input.txt", "../data/day23/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day23/input.txt", "../data/day23/input-part2-answer.txt");
    }
}
