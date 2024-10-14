use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub struct Day8 {}

impl Day8 {
    fn get_memory_chars(line: &str) -> usize {
        let mut chars = line.chars().peekable();
        let mut count = 0;
        let mut in_string = false;

        while let Some(c) = chars.next() {
            match c {
                '"' if !in_string => in_string = true,
                '"' if in_string => break,
                '\\' if in_string => {
                    match chars.next() {
                        Some('\\') | Some('"') => count += 1,
                        Some('x') => {
                            // Consume next two characters (hex)
                            chars.next();
                            chars.next();
                            count += 1;
                        }
                        _ => count += 1, // Invalid escape, count as is
                    }
                }
                _ if in_string => count += 1,
                _ => {}
            }
        }
        count
    }

    fn count_encoded_chars(s: &str) -> usize {
        let mut count = 2; // Start and end quotes

        for c in s.chars() {
            match c {
                '"' | '\\' => count += 2, // These characters need to be escaped
                _ => count += 1,
            }
        }

        count
    }

    pub fn part1() -> Result<usize> {
        let file = File::open("src/day_8/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 8 input file: {e}"));
        let reader = BufReader::new(file);
        let mut total_code_chars = 0;
        let mut total_memory_chars = 0;

        for line in reader.lines() {
            let line = line?;
            let code_chars = line.len();
            let memory_chars = Self::get_memory_chars(&line);

            total_code_chars += code_chars;
            total_memory_chars += memory_chars;
        }
        Ok(total_code_chars - total_memory_chars)
    }

    pub fn part2() -> Result<usize> {
        let file = File::open("src/day_8/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 8 input file: {e}"));
        let reader = BufReader::new(file);
        let mut total_code_chars = 0;
        let mut total_encoded_chars = 0;

        for line in reader.lines() {
            let line = line?;
            let code_chars = line.len();
            let encoded_chars = Self::count_encoded_chars(&line);

            total_encoded_chars += encoded_chars;
            total_code_chars += code_chars;
        }
        Ok(total_encoded_chars - total_code_chars)
    }
}
