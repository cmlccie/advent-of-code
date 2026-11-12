package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 19: Medicine for Rudolph
-------------------------------------------------------------------------------------------------*/

class Day19Test {
    private static final Solution DAY = new Day19();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day19/example.txt", "../data/day19/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day19/example.txt", "../data/day19/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day19/input.txt", "../data/day19/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day19/input.txt", "../data/day19/input-part2-answer.txt");
    }
}
