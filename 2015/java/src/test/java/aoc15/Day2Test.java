package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 2: I Was Told There Would Be No Math
-------------------------------------------------------------------------------------------------*/

class Day2Test {
    private static final Solution DAY = new Day2();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day2/example.txt", "../data/day2/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day2/example.txt", "../data/day2/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day2/input.txt", "../data/day2/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day2/input.txt", "../data/day2/input-part2-answer.txt");
    }
}
