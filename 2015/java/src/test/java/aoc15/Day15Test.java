package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 15: Science for Hungry People
-------------------------------------------------------------------------------------------------*/

class Day15Test {
    private static final Solution DAY = new Day15();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day15/example.txt", "../data/day15/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day15/example.txt", "../data/day15/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day15/input.txt", "../data/day15/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day15/input.txt", "../data/day15/input-part2-answer.txt");
    }
}
