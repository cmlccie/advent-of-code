package aoc15;

import java.util.Optional;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/*-------------------------------------------------------------------------------------------------
  Day 25: Let It Snow
-------------------------------------------------------------------------------------------------*/

public final class Day25 implements Solution {
    @Override
    public String title() {
        return "Day 25: Let It Snow";
    }

    @Override
    public Optional<String> part1(String input) {
        Coordinates targetCoordinates = parseCoordinates(input);

        // BigInteger currentCode = BigInteger.valueOf(20151125);
        long currentCode = 20151125;
        int currentRow = 1;
        int currentColumn = 1;

        int maxRow = 1;

        do {
            currentCode = (currentCode * 252533) % 33554393;

            if (currentRow == 1) {
                maxRow++;
                currentRow = maxRow;
                currentColumn = 1;
            } else {
                currentRow--;
                currentColumn++;
            }
        } while (!(currentRow == targetCoordinates.row() && currentColumn == targetCoordinates.column()));


        return Optional.of(String.valueOf(currentCode));
    }

    @Override
    public Optional<String> part2(String input) {
        return Optional.empty();
    }

    /*--------------------------------------------------------------------------------------
      Core
    --------------------------------------------------------------------------------------*/

    record Coordinates(int row, int column) {}

    static Coordinates parseCoordinates(String s) {
        Pattern pattern = Pattern.compile("row (\\d+), column (\\d+)");
        Matcher matcher = pattern.matcher(s);

        if (!matcher.find()) {
            throw new IllegalArgumentException("Invalid input: " + s);
        }

        int row = Integer.parseInt(matcher.group(1));
        int column = Integer.parseInt(matcher.group(2));
        return new Coordinates(row, column);
    }
}
