package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 16: Aunt Sue
-------------------------------------------------------------------------------------------------*/

class Day16Test {
    private static final Solution DAY = new Day16();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day16/example.txt", "../data/day16/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day16/example.txt", "../data/day16/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day16/input.txt", "../data/day16/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day16/input.txt", "../data/day16/input-part2-answer.txt");
    }
}
