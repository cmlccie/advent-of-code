package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 6: Probably a Fire Hazard
-------------------------------------------------------------------------------------------------*/

class Day6Test {
    private static final Solution DAY = new Day6();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day6/example.txt", "../data/day6/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day6/example.txt", "../data/day6/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day6/input.txt", "../data/day6/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day6/input.txt", "../data/day6/input-part2-answer.txt");
    }
}
