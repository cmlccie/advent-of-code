package aoc15;

import java.util.Optional;

/*-------------------------------------------------------------------------------------------------
  Day 2: I Was Told There Would Be No Math
-------------------------------------------------------------------------------------------------*/

public final class Day2 implements Solution {
    @Override
    public String title() {
        return "Day 2: I Was Told There Would Be No Math";
    }

    @Override
    public Optional<String> part1(String input) {
        int totalPaper = 0;

        for (String line : (Iterable<String>) input.lines()::iterator) {
            Dimensions dims = parseDimensions(line);

            int areaLengthWidth = dims.length() * dims.width();
            int areaWidthHeight = dims.width() * dims.height();
            int areaHeightLength = dims.height() * dims.length();

            int surfaceArea = 2 * (areaLengthWidth + areaWidthHeight + areaHeightLength);
            int extra = Math.min(areaLengthWidth, Math.min(areaWidthHeight, areaHeightLength));

            totalPaper += surfaceArea + extra;
        }

        return Optional.of(String.valueOf(totalPaper));
    }

    @Override
    public Optional<String> part2(String input) {
        int totalRibbon = 0;

        for (String line : (Iterable<String>) input.lines()::iterator) {
            Dimensions dims = parseDimensions(line);

            int perimeterLengthWidth = 2 * (dims.length() + dims.width());
            int perimeterWidthHeight = 2 * (dims.width() + dims.height());
            int perimeterHeightLength = 2 * (dims.height() + dims.length());
            int minimumPerimeter = Math.min(perimeterLengthWidth, Math.min(perimeterWidthHeight, perimeterHeightLength));

            int bow = dims.length() * dims.width() * dims.height();

            totalRibbon += minimumPerimeter + bow;
        }

        return Optional.of(String.valueOf(totalRibbon));
    }

    /*--------------------------------------------------------------------------------------
      Core
    --------------------------------------------------------------------------------------*/

    record Dimensions(int length, int width, int height) {}

    static Dimensions parseDimensions(String s) {
        String[] parts = s.split("x");
        assert parts.length == 3;
        int length = Integer.parseInt(parts[0]);
        int width = Integer.parseInt(parts[1]);
        int height = Integer.parseInt(parts[2]);
        return new Dimensions(length, width, height);
    }
}
