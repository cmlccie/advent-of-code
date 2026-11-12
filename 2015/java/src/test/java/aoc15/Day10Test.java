package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 10: Elves Look, Elves Say
-------------------------------------------------------------------------------------------------*/

class Day10Test {
    private static final Solution DAY = new Day10();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day10/example.txt", "../data/day10/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day10/example.txt", "../data/day10/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day10/input.txt", "../data/day10/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day10/input.txt", "../data/day10/input-part2-answer.txt");
    }
}
