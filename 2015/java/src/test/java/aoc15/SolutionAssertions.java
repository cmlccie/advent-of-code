package aoc15;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import aoc15.shared.Answers;
import aoc15.shared.Inputs;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Optional;
import java.util.function.Function;

/*-------------------------------------------------------------------------------------------------
  Solution Assertions
-------------------------------------------------------------------------------------------------*/

final class SolutionAssertions {
    private SolutionAssertions() {}

    /**
     * Asserts a solution part produces the expected answer. Skips the test (rather than failing)
     * until both the input and answer files exist.
     */
    static void assertSolution(
            Function<String, Optional<String>> part, String inputPath, String answerPath) {
        assumeTrue(Files.exists(Path.of(inputPath)), "missing input: " + inputPath);
        assumeTrue(Files.exists(Path.of(answerPath)), "missing answer: " + answerPath);

        assertEquals(Answers.getAnswer(answerPath), part.apply(Inputs.getInput(inputPath)));
    }
}
