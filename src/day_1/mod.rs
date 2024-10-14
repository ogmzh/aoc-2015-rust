use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

pub struct Day1 {}

impl Day1 {
    pub fn part1() -> Result<i32> {
        let file = File::open("src/day_1/input.txt")?;
        let reader = BufReader::new(file);
        let mut floor = 0;

        for byte in reader.bytes() {
            match byte? as char {
                ')' => floor -= 1,
                '(' => floor += 1,
                _ => {}
            }
        }
        println!("Floor {}", floor);
        Ok(floor)
    }

    pub fn part2() -> Result<usize> {
        let file = File::open("src/day_1/input.txt")?;
        let reader = BufReader::new(file);
        let mut floor = 0;

        let bytes = reader.bytes().enumerate();

        for (i, byte) in bytes {
            if floor < 0 {
                return Ok(i);
            }
            match byte? as char {
                ')' => floor -= 1,
                '(' => floor += 1,
                _ => {}
            }
        }
        Ok(0)
    }
}
