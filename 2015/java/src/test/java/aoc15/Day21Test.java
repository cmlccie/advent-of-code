package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 21: RPG Simulator 20XX
-------------------------------------------------------------------------------------------------*/

class Day21Test {
    private static final Solution DAY = new Day21();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day21/example.txt", "../data/day21/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day21/example.txt", "../data/day21/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day21/input.txt", "../data/day21/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day21/input.txt", "../data/day21/input-part2-answer.txt");
    }
}
