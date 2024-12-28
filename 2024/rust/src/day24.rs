use core::panic;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fmt::{self, Display, Formatter};
use std::fs::read_to_string;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, BitXor};
use std::path::Path;
use std::rc::Rc;

/*-------------------------------------------------------------------------------------------------
  Day 24: Crossed Wires
-------------------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let (mut wires, gates) = parse_input_file(input);
    process_gates(&mut wires, &gates);
    combine_bits(&wires, "z").to_string()
}

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> String {
    let (_, gates) = parse_input_file(input);
    let crossed_wires = find_crossed_wires(&gates);

    let crossed_wires: BTreeSet<Wire> = crossed_wires.iter().flatten().cloned().collect();

    crossed_wires.iter().join(",")
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Wire = Rc<str>;
type InputWire = Wire;
type OutputWire = Wire;
type Inputs = [InputWire; 2];
type Wires = HashMap<Wire, bool>;
type Gates = HashMap<Gate, Wire>;
type Bit = u8;
type Output = u64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    inputs: Inputs,
    operation: GateOperation,
    role: GateRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GateOperation {
    Xor,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GateRole {
    XyXor,
    XyAnd,
    ZXor,
    CAnd,
    COr,
}

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> (Wires, Gates) {
    let input = read_to_string(input).unwrap();

    let blank_line_index = input.lines().position(|line| line.is_empty()).unwrap();

    let wires: Wires = input
        .lines()
        .take(blank_line_index)
        .map(|line| {
            let mut parts = line.split(": ");
            let wire = parts.next().unwrap().into();
            let value = match parts.next().unwrap() {
                "0" => false,
                "1" => true,
                _ => panic!("Invalid value"),
            };
            (wire, value)
        })
        .collect();

    let gates: Gates = input
        .lines()
        .skip(blank_line_index + 1)
        .map(|line| {
            let mut parts = line.split_whitespace();
            let input0 = parts.next().unwrap();
            let operation = parts.next().unwrap();
            let input1 = parts.next().unwrap();
            let _ = parts.next();
            let output = parts.next().unwrap().into();
            (Gate::new(input0, input1, operation), output)
        })
        .collect();

    (wires, gates)
}

fn process_gates(wires: &mut Wires, gates: &Gates) {
    let mut remaining_gates: VecDeque<(Gate, Wire)> = gates
        .iter()
        .map(|(gate, wire)| (gate.clone(), wire.clone()))
        .collect();

    while let Some((gate, out_wire)) = remaining_gates.pop_front() {
        match (wires.get(&gate.inputs[0]), wires.get(&gate.inputs[1])) {
            (Some(input0), Some(input1)) => {
                let output = gate.operation.function()(*input0, *input1);
                wires.insert(out_wire, output);
            }
            _ => remaining_gates.push_back((gate, out_wire)),
        }
    }
}

fn combine_bits(wires: &Wires, selector: &str) -> Output {
    wires
        .iter()
        .filter(|(wire, _)| wire.starts_with(selector))
        .fold(0, |acc, (wire, value)| {
            let bit = wire_bit(wire).unwrap();
            if *value {
                acc | (1 << bit)
            } else {
                acc
            }
        })
}

fn find_crossed_wires(gates: &Gates) -> Vec<Inputs> {
    let mut crossed_wires: Vec<Inputs> = Vec::new();

    // First bit
    let xy_xor_gate = Gate::new("x00", "y00", "XOR");
    let xy_and_gate = Gate::new("x00", "y00", "AND");
    let z_wire: Wire = "z00".into();
    let z_xor_out_wire = gates.get(&xy_xor_gate).unwrap();

    if &z_wire != z_xor_out_wire {
        crossed_wires.push([z_xor_out_wire.clone(), z_wire.clone()]);
    };

    let mut c_wire = gates.get(&xy_and_gate).unwrap().clone();

    // Middle bits
    for bit in 1..45u8 {
        let x_wire: Wire = format!("x{:02}", bit).into();
        let y_wire: Wire = format!("y{:02}", bit).into();
        let z_wire: Wire = format!("z{:02}", bit).into();

        let xy_xor_gate = Gate::from(inputs(x_wire.clone(), y_wire.clone()), "XOR".into());
        let (xy_xor_gate, xy_xor_out_wire) = {
            let (gate, out_wire, crossed_pair) = get_gate(gates, &xy_xor_gate);
            if let Some(crossed_pair) = crossed_pair {
                crossed_wires.push(crossed_pair);
            };
            (gate, out_wire)
        };

        let xy_and_gate = Gate::from(inputs(x_wire.clone(), y_wire.clone()), "AND".into());
        let (xy_and_gate, xy_and_out_wire) = {
            let (gate, out_wire, crossed_pair) = get_gate(gates, &xy_and_gate);
            if let Some(crossed_pair) = crossed_pair {
                crossed_wires.push(crossed_pair);
            };
            (gate, out_wire)
        };

        let z_xor_gate = Gate::from(
            inputs(xy_xor_out_wire.clone(), c_wire.clone()),
            "XOR".into(),
        );
        let (z_xor_gate, z_xor_out_wire) = {
            let (gate, out_wire, crossed_pair) = get_gate(gates, &z_xor_gate);
            if let Some(crossed_pair) = crossed_pair {
                crossed_wires.push(crossed_pair);
            };
            (gate, out_wire)
        };

        if z_wire != z_xor_out_wire {
            crossed_wires.push([z_xor_out_wire.clone(), z_wire.clone()]);
        };

        let c_and_gate = Gate::from(
            inputs(xy_xor_out_wire.clone(), c_wire.clone()),
            "AND".into(),
        );
        let (c_and_gate, c_and_out_wire) = {
            let (gate, out_wire, crossed_pair) = get_gate(gates, &c_and_gate);
            if let Some(crossed_pair) = crossed_pair {
                crossed_wires.push(crossed_pair);
            };
            (gate, out_wire)
        };

        let c_or_gate = Gate::from(
            inputs(xy_and_out_wire.clone(), c_and_out_wire.clone()),
            "OR".into(),
        );
        let (c_or_gate, c_or_out_wire) = {
            let (gate, out_wire, crossed_pair) = get_gate(gates, &c_or_gate);
            if let Some(crossed_pair) = crossed_pair {
                crossed_wires.push(crossed_pair);
            };
            (gate, out_wire)
        };

        log::debug!(
            r#"bit: {bit:02}
            xy_xor: {xy_xor_gate} -> {xy_xor_out_wire}
            xy_and: {xy_and_gate} -> {xy_and_out_wire}
            z_xor:  {z_xor_gate} -> {z_xor_out_wire}
            c_and:  {c_and_gate} -> {c_and_out_wire}
            c_or:   {c_or_gate} -> {c_or_out_wire}
            "#
        );

        c_wire = c_or_out_wire;
    }

    // Last bit
    let z_wire = format!("z{:02}", 45).into();
    if c_wire != z_wire {
        crossed_wires.push([c_wire.clone(), z_wire.clone()]);
    };

    log::debug!("crossed_wires: {:?}", crossed_wires);
    crossed_wires
}

fn get_gate(gates: &Gates, gate: &Gate) -> (Gate, Wire, Option<Inputs>) {
    if let Some(out_wire) = gates.get(gate) {
        // Gate exists; return the gate and current output wire
        return (gate.clone(), out_wire.clone(), None);
    };

    // Gate does not exist = one of the input wires must be crossed

    // See if we can find a matching gate with the first input wire and gate operation
    if let Some((correct_gate, other_input, out_wire)) =
        find_gate(gates, &gate.inputs[0], gate.operation)
    {
        return (
            correct_gate,
            out_wire,
            Some([gate.inputs[1].clone(), other_input.clone()]),
        );
    };

    // See if we can find a matching gate with the second input wire and gate operation
    if let Some((correct_gate, other_input, out_wire)) =
        find_gate(gates, &gate.inputs[1], gate.operation)
    {
        return (
            correct_gate,
            out_wire,
            Some([gate.inputs[0].clone(), other_input.clone()]),
        );
    };

    panic!("Could not find gate! Gate: {:?}", gate);
}

fn find_gate(
    gates: &Gates,
    input: &InputWire,
    operation: GateOperation,
) -> Option<(Gate, InputWire, OutputWire)> {
    gates
        .iter()
        .filter(|(gate, _)| gate.operation == operation)
        .find_map(|(gate, out_wire)| {
            if gate.inputs[0] == *input {
                Some((gate.clone(), gate.inputs[1].clone(), out_wire.clone()))
            } else if gate.inputs[1] == *input {
                Some((gate.clone(), gate.inputs[0].clone(), out_wire.clone()))
            } else {
                None
            }
        })
}

/*-----------------------------------------------------------------------------
  Inputs Functions
-----------------------------------------------------------------------------*/

fn inputs(a: InputWire, b: InputWire) -> Inputs {
    if a < b {
        [a, b]
    } else {
        [b, a]
    }
}

/*-----------------------------------------------------------------------------
  Wire Functions
-----------------------------------------------------------------------------*/

fn wire_bit(wire: &Wire) -> Option<Bit> {
    wire[1..].parse::<Bit>().ok()
}

/*-----------------------------------------------------------------------------
  Gate Implementation
-----------------------------------------------------------------------------*/

impl Gate {
    fn new(input0: &str, input1: &str, operation: &str) -> Self {
        let inputs = inputs(input0.into(), input1.into());
        let operation = operation.into();
        let role = GateRole::new(&inputs, operation);

        Gate {
            inputs,
            operation,
            role,
        }
    }

    fn from(inputs: Inputs, operation: GateOperation) -> Self {
        let role = GateRole::new(&inputs, operation);
        Gate {
            inputs,
            operation,
            role,
        }
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:<3} {:<3} {:<3}",
            self.inputs[0],
            format!("{}", self.operation),
            self.inputs[1],
        )
    }
}

/*-----------------------------------------------------------------------------
  Operation Implementation
-----------------------------------------------------------------------------*/

impl GateOperation {
    fn function(&self) -> fn(bool, bool) -> bool {
        match self {
            GateOperation::And => bool::bitand,
            GateOperation::Or => bool::bitor,
            GateOperation::Xor => bool::bitxor,
        }
    }
}

impl Display for GateOperation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GateOperation::And => "AND",
                GateOperation::Or => "OR",
                GateOperation::Xor => "XOR",
            }
        )
    }
}

impl From<&str> for GateOperation {
    fn from(s: &str) -> Self {
        match s {
            "AND" => GateOperation::And,
            "OR" => GateOperation::Or,
            "XOR" => GateOperation::Xor,
            _ => panic!("Invalid gate type"),
        }
    }
}

/*-----------------------------------------------------------------------------
  Gate Role Implementation
-----------------------------------------------------------------------------*/

impl GateRole {
    fn new(inputs: &Inputs, operation: GateOperation) -> Self {
        let input0 = inputs[0].chars().next().unwrap();
        let input1 = inputs[1].chars().next().unwrap();
        match (input0, input1, operation) {
            ('x', 'y', GateOperation::Xor) => GateRole::XyXor,
            ('x', 'y', GateOperation::And) => GateRole::XyAnd,
            (_, _, GateOperation::Xor) => GateRole::ZXor,
            (_, _, GateOperation::And) => GateRole::CAnd,
            (_, _, GateOperation::Or) => GateRole::COr,
        }
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_example0_part1() {
        assert_eq!(
            part1("../data/day24/example0.txt"),
            solution("../data/day24/example0-part1-answer.txt")
        );
    }

    #[test]
    fn test_example1_part1() {
        assert_eq!(
            part1("../data/day24/example1.txt"),
            solution("../data/day24/example1-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day24/input.txt"),
            solution("../data/day24/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day24/input.txt"),
            solution("../data/day24/input-part2-answer.txt")
        );
    }
}
