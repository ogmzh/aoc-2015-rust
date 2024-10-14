use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read, Result},
};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Location {
    x: i32,
    y: i32,
}

pub struct Day3 {}

impl Day3 {
    pub fn part1() -> Result<HashSet<Location>> {
        let file = File::open("src/day_3/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 3 input file: {e}"));
        let reader = BufReader::new(file);
        let mut santa_location = Location { x: 0, y: 0 };
        let mut visited_locations: HashSet<Location> = HashSet::new();
        visited_locations.insert(santa_location.clone());
        for byte in reader.bytes() {
            match byte? as char {
                '^' => santa_location.y += 1,
                'v' => santa_location.y -= 1,
                '>' => santa_location.x += 1,
                '<' => santa_location.x -= 1,
                _ => println!("Default"),
            }
            visited_locations.insert(santa_location.clone());
        }

        Ok(visited_locations)
    }

    pub fn part2() -> Result<usize> {
        let file = File::open("src/day_3/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 3 input file: {e}"));
        let reader = BufReader::new(file);

        let mut unique_locations: HashSet<Location> = HashSet::new();

        let mut santa_location = Location { x: 0, y: 0 };
        let mut robo_santa_location = Location { x: 0, y: 0 };
        for (i, byte) in reader.bytes().enumerate() {
            match byte? as char {
                '^' => {
                    if i % 2 == 0 {
                        santa_location.y += 1;
                    } else {
                        robo_santa_location.y += 1;
                    };
                }
                'v' => {
                    if i % 2 == 0 {
                        santa_location.y -= 1;
                    } else {
                        robo_santa_location.y -= 1;
                    };
                }
                '>' => {
                    if i % 2 == 0 {
                        santa_location.x += 1;
                    } else {
                        robo_santa_location.x += 1;
                    };
                }
                '<' => {
                    if i % 2 == 0 {
                        santa_location.x -= 1;
                    } else {
                        robo_santa_location.x -= 1;
                    };
                }
                _ => println!("Default"),
            }
            if i % 2 == 0 && !unique_locations.contains(&santa_location) {
                unique_locations.insert(santa_location.clone());
            } else if !unique_locations.contains(&robo_santa_location) {
                unique_locations.insert(robo_santa_location.clone());
            }
        }

        Ok(unique_locations.len())
    }
}
