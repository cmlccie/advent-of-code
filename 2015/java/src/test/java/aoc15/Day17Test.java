package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 17: No Such Thing as Too Much
-------------------------------------------------------------------------------------------------*/

class Day17Test {
    private static final Solution DAY = new Day17();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day17/example.txt", "../data/day17/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day17/example.txt", "../data/day17/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day17/input.txt", "../data/day17/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day17/input.txt", "../data/day17/input-part2-answer.txt");
    }
}
