package aoc15.shared;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.function.UnaryOperator;
import java.util.stream.Stream;

/*-------------------------------------------------------------------------------------------------
  IterTools
-------------------------------------------------------------------------------------------------*/

/**
 * Combinatoric generators modeled on Python's itertools. Each returns a lazy stream of
 * independent, unmodifiable lists, emitted in the same order as Python (lexicographic by
 * position). Elements are treated as unique by position, not by value. Null elements are not
 * supported.
 */

public final class IterTools {
    private IterTools() {}

    /** Cartesian product of the pools, like {@code itertools.product(*pools)}. */
    public static <T> Stream<List<T>> product(List<? extends Collection<? extends T>> pools) {
        List<List<T>> lists = pools.stream().map(p -> List.<T>copyOf(p)).toList();
        int[] sizes = lists.stream().mapToInt(List::size).toArray();
        int[] first = Arrays.stream(sizes).allMatch(s -> s > 0) ? new int[sizes.length] : null;
        return generate(first, a -> nextProduct(a, sizes))
                .map(a -> {
                    List<T> tuple = new ArrayList<>(a.length);
                    for (int i = 0; i < a.length; i++) tuple.add(lists.get(i).get(a[i]));
                    return List.copyOf(tuple);
                });
    }

    /** Product of the pool with itself, like {@code itertools.product(pool, repeat=r)}. */
    public static <T> Stream<List<T>> product(Collection<? extends T> pool, int repeat) {
        requireNonNegative(repeat, "repeat");
        return product(Collections.nCopies(repeat, pool));
    }

    /** Full-length permutations, like {@code itertools.permutations(pool)}. */
    public static <T> Stream<List<T>> permutations(Collection<? extends T> pool) {
        return permutations(pool, pool.size());
    }

    /** r-length permutations, like {@code itertools.permutations(pool, r)}. */
    public static <T> Stream<List<T>> permutations(Collection<? extends T> pool, int r) {
        requireNonNegative(r, "r");
        List<T> items = List.copyOf(pool);
        int n = items.size();
        return generate(r <= n ? range(r) : null, a -> nextPermutation(a, n))
                .map(a -> select(items, a));
    }

    /** r-length combinations, like {@code itertools.combinations(pool, r)}. */
    public static <T> Stream<List<T>> combinations(Collection<? extends T> pool, int r) {
        requireNonNegative(r, "r");
        List<T> items = List.copyOf(pool);
        int n = items.size();
        return generate(r <= n ? range(r) : null, a -> nextCombination(a, n))
                .map(a -> select(items, a));
    }

    /** r-length combinations with repeats, like {@code itertools.combinations_with_replacement}. */
    public static <T> Stream<List<T>> combinationsWithReplacement(
            Collection<? extends T> pool, int r) {
        requireNonNegative(r, "r");
        List<T> items = List.copyOf(pool);
        int n = items.size();
        int[] first = n > 0 || r == 0 ? new int[r] : null;
        return generate(first, a -> nextCombinationWithReplacement(a, n))
                .map(a -> select(items, a));
    }

    /*--------------------------------------------------------------------------------------
      Index generators: each returns the next index tuple as a new array, or null when done.
    --------------------------------------------------------------------------------------*/

    private static int[] nextProduct(int[] prev, int[] sizes) {
        int[] a = prev.clone();
        for (int i = a.length - 1; i >= 0; i--) {
            if (++a[i] < sizes[i]) return a;
            a[i] = 0;
        }
        return null;
    }

    private static int[] nextPermutation(int[] prev, int n) {
        int[] a = prev.clone();
        for (int i = a.length - 1; i >= 0; i--) {
            boolean[] used = new boolean[n];
            for (int j = 0; j < i; j++) used[a[j]] = true;
            int v = a[i] + 1;
            while (v < n && used[v]) v++;
            if (v == n) continue;
            a[i] = v;
            used[v] = true;
            for (int j = i + 1, u = 0; j < a.length; j++) {
                while (used[u]) u++;
                a[j] = u;
                used[u] = true;
            }
            return a;
        }
        return null;
    }

    private static int[] nextCombination(int[] prev, int n) {
        int[] a = prev.clone();
        int r = a.length;
        for (int i = r - 1; i >= 0; i--) {
            if (a[i] == i + n - r) continue;
            a[i]++;
            for (int j = i + 1; j < r; j++) a[j] = a[j - 1] + 1;
            return a;
        }
        return null;
    }

    private static int[] nextCombinationWithReplacement(int[] prev, int n) {
        int[] a = prev.clone();
        for (int i = a.length - 1; i >= 0; i--) {
            if (a[i] == n - 1) continue;
            Arrays.fill(a, i, a.length, a[i] + 1);
            return a;
        }
        return null;
    }

    /*--------------------------------------------------------------------------------------
      Helpers
    --------------------------------------------------------------------------------------*/

    private static Stream<int[]> generate(int[] first, UnaryOperator<int[]> next) {
        return Stream.iterate(first, Objects::nonNull, next);
    }

    private static <T> List<T> select(List<T> items, int[] indices) {
        List<T> tuple = new ArrayList<>(indices.length);
        for (int i : indices) tuple.add(items.get(i));
        return List.copyOf(tuple);
    }

    private static int[] range(int r) {
        int[] a = new int[r];
        for (int i = 0; i < r; i++) a[i] = i;
        return a;
    }

    private static void requireNonNegative(int value, String name) {
        if (value < 0) throw new IllegalArgumentException(name + " must be non-negative: " + value);
    }
}
