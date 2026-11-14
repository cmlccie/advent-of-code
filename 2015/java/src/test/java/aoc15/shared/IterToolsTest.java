package aoc15.shared;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;

/*-------------------------------------------------------------------------------------------------
  Tests: IterTools (expected values from the Python itertools docs)
-------------------------------------------------------------------------------------------------*/

class IterToolsTest {
    private static final List<Character> ABCD = chars("ABCD");

    private static List<Character> chars(String s) {
        return s.chars().mapToObj(c -> (char) c).toList();
    }

    /** Space-separated tuples, e.g. "AB AC" -> [[A, B], [A, C]]. */
    private static List<List<Character>> tuples(String s) {
        return Arrays.stream(s.split(" ")).map(IterToolsTest::chars).toList();
    }

    private static <T> List<List<T>> collect(Stream<List<T>> stream) {
        return stream.toList();
    }

    @Test
    void product() {
        assertEquals(
                tuples("Ax Ay Bx By Cx Cy Dx Dy"),
                collect(IterTools.product(List.of(ABCD, chars("xy")))));
    }

    @Test
    void productRepeat() {
        assertEquals(
                List.of(List.of(0, 0, 0), List.of(0, 0, 1), List.of(0, 1, 0), List.of(0, 1, 1),
                        List.of(1, 0, 0), List.of(1, 0, 1), List.of(1, 1, 0), List.of(1, 1, 1)),
                collect(IterTools.product(List.of(0, 1), 3)));
    }

    @Test
    void productEdgeCases() {
        assertEquals(List.of(List.of()), collect(IterTools.product(List.<List<Integer>>of())));
        assertEquals(List.of(List.of()), collect(IterTools.product(List.of(1, 2), 0)));
        assertEquals(
                List.of(), collect(IterTools.product(List.of(List.of(1), List.<Integer>of()))));
    }

    @Test
    void permutations() {
        assertEquals(
                tuples("AB AC AD BA BC BD CA CB CD DA DB DC"),
                collect(IterTools.permutations(ABCD, 2)));
        assertEquals(
                List.of(List.of(0, 1, 2), List.of(0, 2, 1), List.of(1, 0, 2),
                        List.of(1, 2, 0), List.of(2, 0, 1), List.of(2, 1, 0)),
                collect(IterTools.permutations(List.of(0, 1, 2))));
    }

    @Test
    void permutationsEdgeCases() {
        assertEquals(List.of(List.of()), collect(IterTools.permutations(ABCD, 0)));
        assertEquals(List.of(), collect(IterTools.permutations(ABCD, 5)));
        assertEquals(24, IterTools.permutations(ABCD).count());
    }

    @Test
    void combinations() {
        assertEquals(tuples("AB AC AD BC BD CD"), collect(IterTools.combinations(ABCD, 2)));
        assertEquals(
                List.of(List.of(0, 1, 2), List.of(0, 1, 3), List.of(0, 2, 3), List.of(1, 2, 3)),
                collect(IterTools.combinations(List.of(0, 1, 2, 3), 3)));
    }

    @Test
    void combinationsAreUniqueByPosition() {
        assertEquals(
                List.of(List.of(1, 1), List.of(1, 2), List.of(1, 2)),
                collect(IterTools.combinations(List.of(1, 1, 2), 2)));
    }

    @Test
    void combinationsEdgeCases() {
        assertEquals(List.of(List.of()), collect(IterTools.combinations(ABCD, 0)));
        assertEquals(List.of(), collect(IterTools.combinations(ABCD, 5)));
        assertEquals(tuples("ABCD"), collect(IterTools.combinations(ABCD, 4)));
    }

    @Test
    void combinationsWithReplacement() {
        assertEquals(
                tuples("AA AB AC BB BC CC"),
                collect(IterTools.combinationsWithReplacement(chars("ABC"), 2)));
    }

    @Test
    void combinationsWithReplacementEdgeCases() {
        assertEquals(
                List.of(List.of()), collect(IterTools.combinationsWithReplacement(List.of(), 0)));
        assertEquals(List.of(), collect(IterTools.combinationsWithReplacement(List.of(), 2)));
        assertEquals(35, IterTools.combinationsWithReplacement(ABCD, 4).count());
    }

    @Test
    void negativeLengthThrows() {
        assertThrows(IllegalArgumentException.class, () -> IterTools.combinations(ABCD, -1));
        assertThrows(IllegalArgumentException.class, () -> IterTools.permutations(ABCD, -1));
        assertThrows(
                IllegalArgumentException.class,
                () -> IterTools.combinationsWithReplacement(ABCD, -1));
        assertThrows(IllegalArgumentException.class, () -> IterTools.product(ABCD, -1));
    }
}
