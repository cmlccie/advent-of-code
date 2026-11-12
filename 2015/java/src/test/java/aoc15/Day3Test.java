package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 3: Perfectly Spherical Houses in a Vacuum
-------------------------------------------------------------------------------------------------*/

class Day3Test {
    private static final Solution DAY = new Day3();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day3/example.txt", "../data/day3/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day3/example.txt", "../data/day3/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day3/input.txt", "../data/day3/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day3/input.txt", "../data/day3/input-part2-answer.txt");
    }
}
