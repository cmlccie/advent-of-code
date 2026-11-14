package aoc15;

import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.stream.IntStream;
import java.util.Optional;
import java.util.ArrayList;
import java.util.HashSet;
import aoc15.shared.IterTools;

/*-------------------------------------------------------------------------------------------------
  Day 24: It Hangs in the Balance
-------------------------------------------------------------------------------------------------*/

public final class Day24 implements Solution {
    long groupWeight = 0;

    @Override
    public String title() {
        return "Day 24: It Hangs in the Balance";
    }

    @Override
    public Optional<String> part1(String input) {
        HashSet<Long> gifts = Arrays.stream(input.split("\n"))
            .map(Long::valueOf)
            .collect(() -> new HashSet<>(), HashSet::add, HashSet::addAll);

        groupWeight = gifts.stream().mapToLong(Long::longValue).sum() / 3L;

        Optional<Long> lowestQuantumEntanglement = findLowestQuantumEntanglementOfSmallestGroup(gifts);
        return lowestQuantumEntanglement.map(String::valueOf);
    }

    @Override
    public Optional<String> part2(String input) {
        HashSet<Long> gifts = Arrays.stream(input.split("\n"))
            .map(Long::valueOf)
            .collect(() -> new HashSet<>(), HashSet::add, HashSet::addAll);

        groupWeight = gifts.stream().mapToLong(Long::longValue).sum() / 4L;

        Optional<Long> lowestQuantumEntanglement = findLowestQuantumEntanglementOfSmallestGroup(gifts);
        return lowestQuantumEntanglement.map(String::valueOf);
    }

    /*--------------------------------------------------------------------------------------
      Core
    --------------------------------------------------------------------------------------*/

    Optional<Long> findLowestQuantumEntanglementOfSmallestGroup(HashSet<Long> gifts) {
        for (int groupSize : IntStream.range(1, gifts.size()).toArray()) {
            Optional<Long> minimumQuantumEntanglement = IterTools.combinations(new ArrayList<>(gifts), groupSize)
                .filter(g -> g.stream().mapToLong(Long::longValue).sum() == groupWeight)
                .filter(g -> {
                    HashSet<Long> remainingGifts = new HashSet<>(gifts);
                    remainingGifts.removeAll(g);
                    return findAnyGroup(remainingGifts).isPresent();
                })
                .map(g -> g.stream().reduce(1L, (qe, gift) -> qe * gift))
                .reduce((a, b) -> Math.min(a, b));

            if (minimumQuantumEntanglement.isPresent()) return minimumQuantumEntanglement;
        }
        return Optional.empty();
    }

    // Find any group that sums to the target weight (groupWeight) using a speed optimized recursive search
    Optional<HashSet<Long>> findAnyGroup(HashSet<Long> gifts) {
        // Find any group that sums to the target weight (groupWeight)
        return findAnyGroupRecursive(gifts, new HashSet<>());
    }

    private Optional<HashSet<Long>> findAnyGroupRecursive(HashSet<Long> gifts, HashSet<Long> currentGroup) {
        long currentWeight = currentGroup.stream().mapToLong(Long::longValue).sum();
        long remainingWeight = groupWeight - currentWeight;

        // Base cases
        if (currentWeight == groupWeight) return Optional.of(new HashSet<>(currentGroup));
        if (currentWeight > groupWeight) return Optional.empty();
        if (gifts.isEmpty()) return Optional.empty();

        // Look for the perfect gift: a gift that exactly matches the remaining weight first
        if (gifts.contains(remainingWeight)) {
            currentGroup.add(remainingWeight);
            return Optional.of(new HashSet<>(currentGroup));
        }

        // Otherwise, try adding each gift that does not exceed the remaining weight
        for (long gift : gifts) {
            if (gift <= remainingWeight) {
                HashSet<Long> remainingGifts = new HashSet<>(gifts);
                HashSet<Long> newTestGroup = new HashSet<>(currentGroup);
                remainingGifts.remove(gift);
                newTestGroup.add(gift);
                Optional<HashSet<Long>> result = findAnyGroupRecursive(remainingGifts, newTestGroup);
                if (result.isPresent()) return result;
            }
        }

        return Optional.empty();
    }
}
