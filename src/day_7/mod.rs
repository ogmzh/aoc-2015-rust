use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Day7 {}

#[derive(Debug)]
struct Circuit<'a> {
    // double ended queue because we want to push the assignment instructions to the front
    instructions: &'a mut VecDeque<Instruction>,
    wires: HashMap<String, u16>,
}

impl<'a> Circuit<'a> {
    fn process_and_get(&mut self, key: &str) -> u16 {
        // Part two of the assignment exposed the fact that i designed my structure wrong. I bound the instructions to the circuit,
        // and also making it a VecDeque which i have to mutate and consume
        // instead of having an immutable reference to instructions that can be shared between Circuits. I'm cloning here because i want to be done with it.
        // maybe revisit the assignment in the future cause its fun
        while let Some(instruction) = self.instructions.pop_front() {
            match instruction.operation {
                Operation::ASSIGN => match &instruction.inputs[0] {
                    Input::Value(value) => {
                        self.wires.insert(instruction.output, *value);
                    }
                    Input::Wire(wire) => {
                        if let Some(value) = self.wires.get(wire) {
                            self.wires.insert(instruction.output, *value);
                        } else {
                            self.instructions.push_back(instruction);
                        }
                    }
                },
                Operation::NOT => {
                    if let Input::Wire(wire) = &instruction.inputs[0] {
                        if let Some(value) = self.wires.get(wire) {
                            let result = !value; // Bitwise NOT
                            self.wires.insert(instruction.output, result);
                        } else {
                            self.instructions.push_back(instruction);
                        }
                    }
                }
                Operation::OR => {
                    if let (Input::Wire(wire1), Input::Wire(wire2)) =
                        (&instruction.inputs[0], &instruction.inputs[1])
                    {
                        if let (Some(&value1), Some(&value2)) =
                            (self.wires.get(wire1), self.wires.get(wire2))
                        {
                            let result = value1 | value2;
                            self.wires.insert(instruction.output, result);
                        } else {
                            self.instructions.push_back(instruction);
                            continue;
                        }
                    } else if let (Input::Value(value), Input::Wire(wire)) =
                        (&instruction.inputs[0], &instruction.inputs[1])
                    {
                        if let Some(&wire_value) = self.wires.get(wire) {
                            let result = *value | wire_value;
                            self.wires.insert(instruction.output, result);
                        } else {
                            self.instructions.push_back(instruction);
                            continue;
                        }
                    } else if let (Input::Wire(wire), Input::Value(value)) =
                        (&instruction.inputs[0], &instruction.inputs[1])
                    {
                        if let Some(&wire_value) = self.wires.get(wire) {
                            let result = wire_value | *value;
                            self.wires.insert(instruction.output, result);
                        } else {
                            self.instructions.push_back(instruction);
                            continue;
                        }
                    }
                }
                Operation::AND => {
                    if let (Input::Wire(wire1), Input::Wire(wire2)) =
                        (&instruction.inputs[0], &instruction.inputs[1])
                    {
                        if let (Some(&value1), Some(&value2)) =
                            (self.wires.get(wire1), self.wires.get(wire2))
                        {
                            let result = value1 & value2;
                            self.wires.insert(instruction.output, result);
                        } else {
                            self.instructions.push_back(instruction);
                            continue;
                        }
                    } else if let (Input::Value(value), Input::Wire(wire)) =
                        (&instruction.inputs[0], &instruction.inputs[1])
                    {
                        if let Some(&wire_value) = self.wires.get(wire) {
                            let result = *value & wire_value;
                            self.wires.insert(instruction.output, result);
                        } else {
                            self.instructions.push_back(instruction);
                            continue;
                        }
                    } else if let (Input::Wire(wire), Input::Value(value)) =
                        (&instruction.inputs[0], &instruction.inputs[1])
                    {
                        if let Some(&wire_value) = self.wires.get(wire) {
                            let result = wire_value & *value;
                            self.wires.insert(instruction.output, result);
                        } else {
                            self.instructions.push_back(instruction);
                            continue;
                        }
                    }
                }
                Operation::LSHIFT => {
                    match (&instruction.inputs[0], &instruction.inputs[1]) {
                        (Input::Wire(wire), Input::Value(shift)) => {
                            if let Some(&value) = self.wires.get(wire) {
                                let result = value << shift;
                                self.wires.insert(instruction.output, result);
                            } else {
                                self.instructions.push_back(instruction);
                                continue;
                            }
                        }
                        (Input::Wire(wire1), Input::Wire(wire2)) => {
                            if let (Some(&value), Some(&shift)) =
                                (self.wires.get(wire1), self.wires.get(wire2))
                            {
                                let result = value << shift;
                                self.wires.insert(instruction.output, result);
                            } else {
                                self.instructions.push_back(instruction);
                                continue;
                            }
                        }
                        (Input::Value(value), Input::Wire(wire)) => {
                            if let Some(&shift) = self.wires.get(wire) {
                                let result = *value << shift;
                                self.wires.insert(instruction.output, result);
                            } else {
                                self.instructions.push_back(instruction);
                                continue;
                            }
                        }
                        _ => {
                            // Invalid input combination
                            continue;
                        }
                    }
                }
                Operation::RSHIFT => {
                    match (&instruction.inputs[0], &instruction.inputs[1]) {
                        (Input::Wire(wire), Input::Value(shift)) => {
                            if let Some(&value) = self.wires.get(wire) {
                                let result = value >> shift;
                                self.wires.insert(instruction.output, result);
                            } else {
                                self.instructions.push_back(instruction);
                                continue;
                            }
                        }
                        (Input::Wire(wire1), Input::Wire(wire2)) => {
                            if let (Some(&value), Some(&shift)) =
                                (self.wires.get(wire1), self.wires.get(wire2))
                            {
                                let result = value >> shift;
                                self.wires.insert(instruction.output, result);
                            } else {
                                self.instructions.push_back(instruction);
                                continue;
                            }
                        }
                        (Input::Value(value), Input::Wire(wire)) => {
                            if let Some(&shift) = self.wires.get(wire) {
                                let result = *value >> shift;
                                self.wires.insert(instruction.output, result);
                            } else {
                                self.instructions.push_back(instruction);
                                continue;
                            }
                        }
                        _ => {
                            // Invalid input combination
                            continue;
                        }
                    }
                }
            }
        }
        *self
            .wires
            .get(key)
            .unwrap_or_else(|| panic!("panic while getting the final result"))
    }

    fn new(instructions: &'a mut VecDeque<Instruction>) -> Self {
        Self {
            instructions,
            wires: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    inputs: Vec<Input>,
    operation: Operation,
    output: String,
}

#[derive(Clone, Debug)]
enum Input {
    Wire(String),
    Value(u16),
}

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum Operation {
    ASSIGN,
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
}

impl Day7 {
    fn parse_line(line: String) -> Option<Instruction> {
        let parts = line.split(" -> ").collect::<Vec<&str>>();

        if parts.len() != 2 {
            panic!("Invalid number of string parts");
        }

        let input = parts[0];
        let output = parts[1].to_string();

        let input_parts = input.split_whitespace().collect::<Vec<&str>>();

        match input_parts.len() {
            1 => {
                if let Ok(value) = input_parts[0].parse::<u16>() {
                    Some(Instruction {
                        inputs: vec![Input::Value(value)],
                        operation: Operation::ASSIGN,
                        output,
                    })
                } else {
                    Some(Instruction {
                        inputs: vec![Input::Wire(input_parts[0].to_string())],
                        operation: Operation::ASSIGN,
                        output,
                    })
                }
            }
            2 => {
                if input_parts[0] == "NOT" {
                    Some(Instruction {
                        operation: Operation::NOT,
                        output,
                        // assuming the only 2 part instruction is a NOT
                        inputs: vec![Input::Wire(input_parts[1].to_string())],
                    })
                } else {
                    panic!("unhandled 2 word input {input}");
                }
            }
            3 => {
                let input = if let Ok(value) = input_parts[0].parse::<u16>() {
                    Input::Value(value)
                } else {
                    Input::Wire(input_parts[0].to_string())
                };
                let operation = match input_parts[1] {
                    "OR" => Operation::OR,
                    "AND" => Operation::AND,
                    "LSHIFT" => Operation::LSHIFT,
                    "RSHIFT" => Operation::RSHIFT,
                    _ => panic!("Unknown operation parsing: {}", input_parts[1]),
                };
                let input_2 = if let Ok(value) = input_parts[2].parse::<u16>() {
                    Input::Value(value)
                } else {
                    Input::Wire(input_parts[2].to_string())
                };

                Some(Instruction {
                    inputs: vec![input, input_2],
                    output,
                    operation,
                })
            }
            _ => None,
        }
    }

    pub fn part1() -> u16 {
        let file = File::open("src/day_7/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 5 input file: {e}"));
        let reader = BufReader::new(file);

        let mut instructions = VecDeque::new();
        for line in reader.lines() {
            match line {
                Ok(text) => {
                    if let Some(instruction) = Self::parse_line(text) {
                        match instruction.operation {
                            Operation::ASSIGN => instructions.push_front(instruction),
                            _ => instructions.push_back(instruction),
                        }
                    } else {
                        panic!("Did not parse the instruction line!")
                    }
                }
                _ => panic!("Err reading line"),
            }
        }
        let mut circuit = Circuit::new(&mut instructions);
        circuit.process_and_get("a")
    }

    pub fn part2() -> u16 {
        let file = File::open("src/day_7/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 5 input file: {e}"));
        let reader = BufReader::new(file);

        let mut instructions = VecDeque::new();
        for line in reader.lines() {
            match line {
                Ok(text) => {
                    if let Some(instruction) = Self::parse_line(text) {
                        match instruction.operation {
                            Operation::ASSIGN => instructions.push_front(instruction),
                            _ => instructions.push_back(instruction),
                        }
                    } else {
                        panic!("Did not parse the instruction line!")
                    }
                }
                _ => panic!("Err reading line"),
            }
        }
        let mut instructions_2 = instructions.clone();
        let mut_instructions_ref = &mut instructions;

        let mut circuit_1 = Circuit::new(mut_instructions_ref);
        let c1_result = circuit_1.process_and_get("a");

        for instruction in instructions_2.iter_mut() {
            if instruction.output == "b" {
                *instruction = Instruction {
                    inputs: vec![Input::Value(c1_result)],
                    operation: Operation::ASSIGN,
                    output: "b".to_string(),
                };
                break;
            }
        }
        let mut_instructions_2_ref = &mut instructions_2;
        let mut circuit_2 = Circuit::new(mut_instructions_2_ref);
        circuit_2.process_and_get("a")
    }
}
