package aoc15;

import static aoc15.SolutionAssertions.assertSolution;

import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: Day 5: Doesn't He Have Intern-Elves For This?
-------------------------------------------------------------------------------------------------*/

class Day5Test {
    private static final Solution DAY = new Day5();

    @Test
    void testExampleSolutionPart1() {
        assertSolution(
                DAY::part1, "../data/day5/example.txt", "../data/day5/example-part1-answer.txt");
    }

    @Test
    void testExampleSolutionPart2() {
        assertSolution(
                DAY::part2, "../data/day5/example.txt", "../data/day5/example-part2-answer.txt");
    }

    @Test
    void testPart1Solution() {
        assertSolution(
                DAY::part1, "../data/day5/input.txt", "../data/day5/input-part1-answer.txt");
    }

    @Test
    void testPart2Solution() {
        assertSolution(
                DAY::part2, "../data/day5/input.txt", "../data/day5/input-part2-answer.txt");
    }
}
