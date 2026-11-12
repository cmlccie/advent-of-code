package aoc15;

import aoc15.shared.Inputs;
import java.util.List;
import java.util.Optional;
import java.util.function.Function;
import java.util.stream.IntStream;

/*-------------------------------------------------------------------------------------------------
  Advent of Code 2015
-------------------------------------------------------------------------------------------------*/

public final class Main {
    private Main() {}

    /*--------------------------------------------------------------------------------------
      Solutions
    --------------------------------------------------------------------------------------*/

    static final List<Solution> SOLUTIONS = List.of(
            new Day1(), new Day2(), new Day3(), new Day4(), new Day5(),
            new Day6(), new Day7(), new Day8(), new Day9(), new Day10(),
            new Day11(), new Day12(), new Day13(), new Day14(), new Day15(),
            new Day16(), new Day17(), new Day18(), new Day19(), new Day20(),
            new Day21(), new Day22(), new Day23(), new Day24(), new Day25());

    /*--------------------------------------------------------------------------------------
      Command Line Interface (CLI)
    --------------------------------------------------------------------------------------*/

    private static final String USAGE = """
            Advent of Code 2015

            Usage: aoc15 <day> <part> <input>

              day     day1 .. day25
              part    part1 | part2
              input   path to the puzzle input file

            Example: aoc15 day1 part1 data/day1/input.txt
            """;

    public static void main(String[] args) {
        if (args.length != 3) {
            exit(USAGE);
        }

        Solution solution = parseDay(args[0]).orElseGet(() -> exit("Unknown day: " + args[0]));
        Function<String, Optional<String>> part = switch (args[1]) {
            case "part1" -> solution::part1;
            case "part2" -> solution::part2;
            default -> exit("Unknown part: " + args[1]);
        };

        part.apply(Inputs.getInput(args[2]))
                .ifPresentOrElse(
                        answer -> System.out.println("Answer: " + answer),
                        () -> System.err.println("No answer"));
    }

    private static Optional<Solution> parseDay(String day) {
        return IntStream.rangeClosed(1, SOLUTIONS.size())
                .filter(n -> day.equals("day" + n))
                .mapToObj(n -> SOLUTIONS.get(n - 1))
                .findFirst();
    }

    private static <T> T exit(String message) {
        System.err.println(message);
        System.exit(2);
        throw new IllegalStateException("unreachable");
    }
}
