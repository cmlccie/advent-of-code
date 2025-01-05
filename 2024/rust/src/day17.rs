use regex::Regex;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use itertools::Itertools;

/*-------------------------------------------------------------------------------------------------
  Day 17: Chronospatial Computer
-------------------------------------------------------------------------------------------------*/

fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let (registers, program) = parse_input_file(input);
    let mut computer = Computer::new(registers, &program);
    computer.run();

    Some(computer.format_output())
}

fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let (registers, program) = parse_input_file(input);

    let new_registers = [0, registers[1], registers[2]];
    let register_a = register_a_solver(new_registers, &program, 1).unwrap();

    Some(register_a.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Instruction = u8;
type Operand = u8;
type LiteralOperand = Operand;
type ComboOperand = Operand;
type RegisterValue = u64;
type Pointer = usize;
type Program = Vec<u8>;
type Output = Vec<u8>;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> ([RegisterValue; 3], Program) {
    let input = read_to_string(input).unwrap();

    let input_regex = Regex::new(
        r#"(?x)
        Register\sA:\s(?P<register_a>\d+)\n
        Register\sB:\s(?P<register_b>\d+)\n
        Register\sC:\s(?P<register_c>\d+)\n
        \n
        Program:\s(?P<program>(?:\d,?)+)
    "#,
    )
    .unwrap();

    let parsed_input = input_regex.captures(&input).unwrap();

    let registers = [
        parsed_input["register_a"].parse().unwrap(),
        parsed_input["register_b"].parse().unwrap(),
        parsed_input["register_c"].parse().unwrap(),
    ];

    let program = parsed_input["program"]
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    (registers, program)
}

fn register_a_solver(
    registers: [RegisterValue; 3],
    program: &Program,
    slice_size: usize,
) -> Option<RegisterValue> {
    let register_a = registers[0];
    let program_tail = &program[program.len() - slice_size..];
    for register_octal in 0..63u8 {
        let test_register_a = (register_a << 3) + register_octal as RegisterValue;

        let mut computer = Computer::new([test_register_a, registers[1], registers[2]], program);
        computer.run();

        if computer.output == program_tail {
            let next_registers = [test_register_a, registers[1], registers[2]];
            if slice_size == program.len() {
                return Some(test_register_a);
            } else {
                let result = register_a_solver(next_registers, program, slice_size + 1);
                if result.is_some() {
                    return result;
                } else {
                    continue;
                }
            }
        }
    }

    None
}

/*-----------------------------------------------------------------------------
  Computer
-----------------------------------------------------------------------------*/

struct Computer<'p> {
    register_a: RegisterValue,
    register_b: RegisterValue,
    register_c: RegisterValue,

    program: &'p Program,
    instruction_pointer: Pointer,

    output: Output,
}

impl<'p> Computer<'p> {
    fn new(registers: [RegisterValue; 3], program: &'p Program) -> Self {
        Self {
            register_a: registers[0],
            register_b: registers[1],
            register_c: registers[2],

            program,
            instruction_pointer: 0,

            output: Vec::new(),
        }
    }

    fn increment_instruction_pointer(&mut self) {
        self.instruction_pointer += 2;
    }

    fn format_output(&self) -> String {
        self.output
            .iter()
            .map(|output| output.to_string())
            .join(",")
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];
            self.execute(opcode, operand);
        }
    }

    /*-------------------------------------------------------------------------
      Operands
    -------------------------------------------------------------------------*/

    fn literal_operand(&self, operand: LiteralOperand) -> RegisterValue {
        match operand {
            0..=7 => operand as RegisterValue,
            _ => panic!("Invalid literal operand: {}", operand),
        }
    }

    fn combo_operand(&self, operand: ComboOperand) -> RegisterValue {
        match operand {
            0..=3 => operand as RegisterValue,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Literal operand 7 is reserved and should not appear in programs."),
            _ => panic!("Invalid literal operand: {}", operand),
        }
    }

    /*-------------------------------------------------------------------------
      Instructions
    -------------------------------------------------------------------------*/

    fn execute(&mut self, opcode: Instruction, operand: Operand) {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }

    fn adv(&mut self, operand: ComboOperand) {
        let operand_value = self.combo_operand(operand);
        self.log_execution("adv", 0, operand, operand_value, "A DIVIDE 2^Combo STORE A");
        let base: RegisterValue = 2;
        self.register_a /= base.pow(u32::try_from(operand_value).unwrap());
        self.increment_instruction_pointer();
    }

    fn bxl(&mut self, operand: LiteralOperand) {
        let operand_value = self.literal_operand(operand);
        self.log_execution("bxl", 1, operand, operand_value, "B XOR Literal");
        self.register_b ^= operand_value;
        self.increment_instruction_pointer();
    }

    fn bst(&mut self, operand: ComboOperand) {
        let operand_value = self.combo_operand(operand);
        self.log_execution("bst", 2, operand, operand_value, "B STORE Combo % 8");
        self.register_b = operand_value % 8;
        self.increment_instruction_pointer();
    }

    fn jnz(&mut self, operand: LiteralOperand) {
        let operand_value = self.literal_operand(operand);
        self.log_execution("jnz", 3, operand, operand_value, "JUMP if A is NOT ZERO");
        if self.register_a == 0 {
            self.increment_instruction_pointer();
        } else {
            self.instruction_pointer = usize::try_from(operand_value).unwrap();
        }
    }

    fn bxc(&mut self, _operand: Operand) {
        self.log_execution("bxc", 4, 0, 0, "B XOR C STORE B");
        self.register_b ^= self.register_c;
        self.increment_instruction_pointer();
    }

    fn out(&mut self, operand: ComboOperand) {
        let operand_value = self.combo_operand(operand);
        self.log_execution("out", 5, operand, operand_value, "OUT Combo % 8");
        let output_value = operand_value % 8;
        let output_value = u8::try_from(output_value).unwrap();
        self.output.push(output_value);
        self.increment_instruction_pointer();
    }

    fn bdv(&mut self, operand: ComboOperand) {
        let operand_value = self.combo_operand(operand);
        self.log_execution("bdv", 6, operand, operand_value, "A DIVIDE 2^Combo STORE B");
        let base: RegisterValue = 2;
        self.register_b = self.register_a / base.pow(u32::try_from(operand_value).unwrap());
        self.increment_instruction_pointer();
    }

    fn cdv(&mut self, operand: ComboOperand) {
        let operand_value = self.combo_operand(operand);
        self.log_execution("cdv", 7, operand, operand_value, "A DIVIDE 2^Combo STORE C");
        let base: RegisterValue = 2;
        self.register_c = self.register_a / base.pow(u32::try_from(operand_value).unwrap());
        self.increment_instruction_pointer();
    }

    /*-------------------------------------------------------------------------
      Observability
    -------------------------------------------------------------------------*/

    fn log_execution(
        &self,
        instruction: &str,
        opcode: Instruction,
        operand: Operand,
        operand_value: RegisterValue,
        description: &str,
    ) {
        log::debug!(
            "A: {:14}, B: {:14}, C: {:14} | {}, {}: {}({:14})   // {}",
            self.register_a,
            self.register_b,
            self.register_c,
            opcode,
            operand,
            instruction,
            operand_value,
            description
        );
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 17: Chronospatial Computer")]
pub enum Args {
    Part1 { input: PathBuf },
    Part2 { input: PathBuf },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 { input } => part1(&input),
        Args::Part2 { input } => part2(&input),
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::answers::answer;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1("../data/day17/example0.txt"),
            answer("../data/day17/example0-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day17/input.txt"),
            answer("../data/day17/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2("../data/day17/example1.txt"),
            answer("../data/day17/example1-part2-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day17/input.txt"),
            answer("../data/day17/input-part2-answer.txt")
        );
    }
}
