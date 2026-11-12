package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 7: Some Assembly Required
-------------------------------------------------------------------------------------------------*/

class Day7Test {
    private static final Solution DAY = new Day7();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day7/example.txt", "../data/day7/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day7/example.txt", "../data/day7/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day7/input.txt", "../data/day7/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day7/input.txt", "../data/day7/input-part2-answer.txt");
    }
}
