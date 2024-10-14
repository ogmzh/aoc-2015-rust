use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Instruction {
    command: Command,
    from: (usize, usize),
    to: (usize, usize),
}

pub struct Day6 {}

impl Day6 {
    fn parse_line(line: String) -> Result<Instruction, String> {
        let mut parts = line.split_whitespace();
        let command = match parts.next() {
            Some("toggle") => Command::Toggle,
            Some("turn") => match parts.next() {
                Some("on") => Command::TurnOn,
                Some("off") => Command::TurnOff,
                _ => return Err("Invalid part of TURN command".to_string()),
            },
            _ => return Err("Invalid command".to_string()),
        };

        let coords: Vec<usize> = parts
            .filter(|&word| word != "through")
            .flat_map(|word| word.split(','))
            .map(|val| val.parse::<usize>().unwrap())
            .collect();

        Ok(Instruction {
            command,
            from: (coords[0], coords[1]),
            to: (coords[2], coords[3]),
        })
    }

    fn process_instruction_part2(
        mut matrix: Vec<Vec<u32>>,
        instruction: Instruction,
    ) -> Vec<Vec<u32>> {
        match instruction.command {
            Command::Toggle => {
                let row_slice = &mut matrix[instruction.from.0..=instruction.to.0];
                for row in row_slice {
                    let col_slice = &mut row[instruction.from.1..=instruction.to.1];
                    col_slice.iter_mut().for_each(|val| *val += 2);
                }
            }
            Command::TurnOff => {
                let row_slice = &mut matrix[instruction.from.0..=instruction.to.0];
                for row in row_slice {
                    let col_slice = &mut row[instruction.from.1..=instruction.to.1];
                    col_slice
                        .iter_mut()
                        .for_each(|val| *val = if *val == 0 { 0 } else { *val - 1 });
                }
            }
            Command::TurnOn => {
                let row_slice = &mut matrix[instruction.from.0..=instruction.to.0];
                for row in row_slice {
                    let col_slice = &mut row[instruction.from.1..=instruction.to.1];
                    col_slice.iter_mut().for_each(|val| *val += 1);
                }
            }
        }
        matrix
    }

    fn process_instruction(mut matrix: Vec<Vec<u32>>, instruction: Instruction) -> Vec<Vec<u32>> {
        match instruction.command {
            Command::Toggle => {
                let row_slice = &mut matrix[instruction.from.0..=instruction.to.0];
                for row in row_slice {
                    let col_slice = &mut row[instruction.from.1..=instruction.to.1];
                    col_slice
                        .iter_mut()
                        .for_each(|val| *val = if *val == 0 { 1 } else { 0 });
                }
            }
            Command::TurnOff => {
                let row_slice = &mut matrix[instruction.from.0..=instruction.to.0];
                for row in row_slice {
                    let col_slice = &mut row[instruction.from.1..=instruction.to.1];
                    col_slice.iter_mut().for_each(|val| *val = 0);
                }
            }
            Command::TurnOn => {
                let row_slice = &mut matrix[instruction.from.0..=instruction.to.0];
                for row in row_slice {
                    let col_slice = &mut row[instruction.from.1..=instruction.to.1];
                    col_slice.iter_mut().for_each(|val| *val = 1);
                }
            }
        }
        matrix
    }

    pub fn part1() -> Result<usize, String> {
        let file = File::open("src/day_6/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 5 input file: {e}"));
        let reader = BufReader::new(file);
        let mut matrix: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
        // let mat2 = vec![vec![1, 2, 3, 4], vec![6, 4, 4, 2], vec![61, 62, 53, 4]];
        // println!("{:?}", mat2[1]);
        for line in reader.lines() {
            match line {
                Ok(text) => {
                    let instruction = Self::parse_line(text)?;
                    matrix = Self::process_instruction(matrix, instruction);
                }
                Err(err) => panic!("could not read line: {err}"),
            }
        }

        Ok(matrix
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&value| value == 1)
            .count())
    }

    pub fn part2() -> Result<u32, String> {
        let file = File::open("src/day_6/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 5 input file: {e}"));
        let reader = BufReader::new(file);
        let mut matrix: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

        for line in reader.lines() {
            match line {
                Ok(text) => {
                    let instruction = Self::parse_line(text)?;
                    matrix = Self::process_instruction_part2(matrix, instruction);
                }
                Err(err) => panic!("could not read line: {err}"),
            }
        }

        Ok(matrix.iter().flat_map(|row| row.iter()).sum())
    }
}

#[derive(Debug)]
enum Command {
    Toggle,
    TurnOn,
    TurnOff,
}
