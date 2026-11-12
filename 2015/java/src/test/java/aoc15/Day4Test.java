package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 4: The Ideal Stocking Stuffer
-------------------------------------------------------------------------------------------------*/

class Day4Test {
    private static final Solution DAY = new Day4();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day4/example.txt", "../data/day4/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day4/example.txt", "../data/day4/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day4/input.txt", "../data/day4/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day4/input.txt", "../data/day4/input-part2-answer.txt");
    }
}
