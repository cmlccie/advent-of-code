package aoc15.shared;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.file.Files;
import java.nio.file.Path;

/*-------------------------------------------------------------------------------------------------
  Inputs
-------------------------------------------------------------------------------------------------*/

public final class Inputs {
    private Inputs() {}

    public static String getInput(Path filePath) {
        try {
            return Files.readString(filePath);
        } catch (IOException e) {
            throw new UncheckedIOException(e);
        }
    }

    public static String getInput(String filePath) {
        return getInput(Path.of(filePath));
    }
}
