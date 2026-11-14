package aoc15;

import java.util.List;
import java.util.Optional;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/*-------------------------------------------------------------------------------------------------
  Day 23: Opening the Turing Lock
-------------------------------------------------------------------------------------------------*/

public final class Day23 implements Solution {
    @Override
    public String title() {
        return "Day 23: Opening the Turing Lock";
    }

    @Override
    public Optional<String> part1(String input) {
        List<ProgramInstruction> program = input.lines().map(ProgramInstruction::parse).toList();
        State state = new State(0, 0, 0);
        while (state.nextInstruction < program.size()) {
            state = program.get(state.nextInstruction).apply(state);
        }
        return Optional.of(String.valueOf(state.b));
    }

    @Override
    public Optional<String> part2(String input) {
        List<ProgramInstruction> program = input.lines().map(ProgramInstruction::parse).toList();
        State state = new State(1, 0, 0);
        while (state.nextInstruction < program.size()) {
            state = program.get(state.nextInstruction).apply(state);
        }
        return Optional.of(String.valueOf(state.b));
    }

    /*--------------------------------------------------------------------------------------
      Core
    --------------------------------------------------------------------------------------*/

    record State(int a, int b, int nextInstruction) {
        int value(char register) {
            return switch (register) {
                case 'a' -> a;
                case 'b' -> b;
                default -> throw new IllegalArgumentException("Unknown register: " + register);
            };
        }

        State withValue(char register, int value) {
            return switch (register) {
                case 'a' -> new State(value, b, nextInstruction);
                case 'b' -> new State(a, value, nextInstruction);
                default -> throw new IllegalArgumentException("Unknown register: " + register);
            };
        }

        State jump(int delta) {
            return new State(a, b, nextInstruction + delta);
        }
    }

    record ProgramInstruction(Instruction opcode, Character register, Integer offset) {
        private static final Pattern INSTRUCTION_PATTERN = Pattern.compile(
            "^(hlf|tpl|inc|jmp|jie|jio) (?:(?:([ab])(?:, ([+-]\\d+))?)|([+-]\\d+))$"
        );

        static ProgramInstruction parse(String text) {
            Matcher matcher = INSTRUCTION_PATTERN.matcher(text);
            if (matcher.matches()) {
                Instruction opcode = Instruction.valueOf(matcher.group(1).toUpperCase());
                Character register = matcher.group(2) == null ? null : matcher.group(2).charAt(0);
                String offsetText = matcher.group(3) == null ? matcher.group(4) : matcher.group(3);
                Integer offset = offsetText == null ? null : Integer.parseInt(offsetText);
                return new ProgramInstruction(opcode, register, offset);
            }

            throw new IllegalArgumentException("Could not parse instruction: " + text);
        }

        State apply(State state) {
            return opcode.apply(state, register, offset);
        }
    }

    enum Instruction {
        HLF((state, register, offset) -> state.withValue(register, state.value(register) / 2).jump(1)),
        TPL((state, register, offset) -> state.withValue(register, state.value(register) * 3).jump(1)),
        INC((state, register, offset) -> state.withValue(register, state.value(register) + 1).jump(1)),
        JMP((state, register, offset) -> state.jump(offset)),
        JIE((state, register, offset) -> state.value(register) % 2 == 0 ? state.jump(offset) : state.jump(1)),
        JIO((state, register, offset) -> state.value(register) == 1 ? state.jump(offset) : state.jump(1));

        private final Operation operation;

        Instruction(Operation operation) {
            this.operation = operation;
        }

        State apply(State state, Character register, Integer offset) {
            return operation.apply(state, register, offset);
        }

        @FunctionalInterface
        private interface Operation {
            State apply(State state, Character register, Integer offset);
        }
    }
}
