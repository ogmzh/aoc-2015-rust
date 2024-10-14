use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Result},
};

static VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

pub fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)
}

pub struct Day5 {
    naughty_strings: HashSet<&'static str>,
}

pub enum SantaString {
    Naughty,
    Nice,
}

impl Default for Day5 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day5 {
    pub fn new() -> Self {
        let mut naughty_strings = HashSet::new();
        naughty_strings.insert("ab");
        naughty_strings.insert("cd");
        naughty_strings.insert("pq");
        naughty_strings.insert("xy");

        Day5 { naughty_strings }
    }

    fn is_string_naughty_or_nice(&self, line: String) -> SantaString {
        let mut existing_chars: HashMap<usize, char> = HashMap::new();
        let mut is_double_letter = false;
        for (index, window) in line.chars().collect::<Vec<char>>().windows(2).enumerate() {
            if self
                .naughty_strings
                .contains(format!("{}{}", window[0], window[1]).as_str())
            {
                return SantaString::Naughty;
            }

            if is_vowel(window[0]) && !existing_chars.contains_key(&index) {
                existing_chars.insert(index, window[0]);
            }

            // in case our string is "hello" we want to add the 2nd char of our window (e)
            // which will then exist on index 1 and wont get reinserted in the next iteration on block above
            if is_vowel(window[1]) && !existing_chars.contains_key(&(index + 1)) {
                existing_chars.insert(index + 1, window[1]);
            }

            if !is_double_letter && window[0] == window[1] {
                is_double_letter = true;
            }
        }

        if is_double_letter && existing_chars.len() > 2 {
            println!("{}::::existing vowels {}", line, existing_chars.len());
            return SantaString::Nice;
        }

        SantaString::Naughty
    }

    pub fn part1(&self) -> Result<i32> {
        let file = File::open("src/day_5/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 5 input file: {e}"));
        let reader = BufReader::new(file);

        let mut nice_count = 0;
        for line_res in reader.lines() {
            match line_res {
                Ok(line) => {
                    if let SantaString::Nice = self.is_string_naughty_or_nice(line) {
                        nice_count += 1
                    }
                }
                Err(err) => {
                    eprintln!("error reading line: {err}");
                    return Err(err);
                }
            }
        }

        Ok(nice_count)
    }
    fn is_string_nice_part2(&self, line: &str) -> SantaString {
        let chars: Vec<char> = line.chars().collect();

        // Check for pair of letters appearing twice without overlapping
        let has_repeating_pair = 'outer: {
            for i in 0..chars.len() - 1 {
                let pair = (chars[i], chars[i + 1]);
                for j in i + 2..chars.len() - 1 {
                    if (chars[j], chars[j + 1]) == pair {
                        break 'outer true;
                    }
                }
            }
            false
        };

        // Check for letter repeating with exactly one letter between
        let has_repeating_letter_with_gap = chars.windows(3).any(|w| w[0] == w[2]);

        if has_repeating_pair && has_repeating_letter_with_gap {
            SantaString::Nice
        } else {
            SantaString::Naughty
        }
    }

    pub fn part2(&self) -> Result<i32> {
        let file = File::open("src/day_5/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 5 input file: {e}"));
        let reader = BufReader::new(file);
        let nice_count = reader
            .lines()
            .map_while(Result::ok)
            // matches! macro is a replacement for my if let from line 82
            .filter(|line| matches!(self.is_string_nice_part2(line), SantaString::Nice))
            .count();
        Ok(nice_count as i32)
    }
}
