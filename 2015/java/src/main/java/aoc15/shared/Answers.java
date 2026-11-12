package aoc15.shared;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Optional;

/*-------------------------------------------------------------------------------------------------
  Answers
-------------------------------------------------------------------------------------------------*/

public final class Answers {
    private Answers() {}

    public static Optional<String> getAnswer(Path filePath) {
        try {
            return Optional.of(Files.readString(filePath).strip());
        } catch (IOException e) {
            return Optional.empty();
        }
    }

    public static Optional<String> getAnswer(String filePath) {
        return getAnswer(Path.of(filePath));
    }
}
