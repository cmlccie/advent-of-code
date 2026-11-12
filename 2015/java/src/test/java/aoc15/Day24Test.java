package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 24: It Hangs in the Balance
-------------------------------------------------------------------------------------------------*/

class Day24Test {
    private static final Solution DAY = new Day24();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day24/example.txt", "../data/day24/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day24/example.txt", "../data/day24/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day24/input.txt", "../data/day24/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day24/input.txt", "../data/day24/input-part2-answer.txt");
    }
}
