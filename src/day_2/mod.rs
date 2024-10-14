use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

#[derive(Debug)]
struct GiftBox {
    length: u32,
    width: u32,
    height: u32,
}

impl GiftBox {
    fn smallest_sides(&self) -> (u32, u32) {
        if self.length >= self.width && self.length >= self.height {
            return (self.width, self.height);
        } else if self.width >= self.length && self.width >= self.height {
            return (self.length, self.height);
        } else if self.height >= self.length && self.height >= self.width {
            return (self.length, self.width);
        }
        (self.height, self.width)
    }

    fn total_surface_area(&self) -> u32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn slack_area(&self) -> u32 {
        let (s1, s2) = self.smallest_sides();
        s1 * s2
    }
}

pub struct Day2 {}

impl Day2 {
    fn get_gift_boxes() -> Vec<GiftBox> {
        let file = File::open("src/day_2/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 2 input file: {e}"));
        let reader = BufReader::new(file);
        let mut gift_boxes: Vec<GiftBox> = vec![];
        for line in reader.lines() {
            match line {
                Ok(text) => {
                    let parts = text.split('x').collect::<Vec<&str>>();
                    let gift_box = GiftBox {
                        length: parts.first().unwrap().parse().unwrap(),
                        width: parts.get(1).unwrap().parse().unwrap(),
                        height: parts.get(2).unwrap().parse::<u32>().unwrap(),
                    };
                    gift_boxes.push(gift_box);
                }
                Err(err) => eprintln!("Err reading line: {:?}", err),
            }
        }
        gift_boxes
    }

    pub fn part1() -> Result<u32> {
        let boxes = Day2::get_gift_boxes();
        let total = boxes
            .iter()
            .map(|gift_box| gift_box.total_surface_area() + gift_box.slack_area())
            .sum();
        Ok(total)
    }

    pub fn part2() -> Result<u32> {
        let boxes = Day2::get_gift_boxes();
        let ribbon_total = boxes
            .iter()
            .map(|gift_box| {
                let (a, b) = gift_box.smallest_sides();
                println!("Box: {:?}", gift_box);
                let ribbon_length =
                    (a + a + b + b) + (gift_box.length * gift_box.width * gift_box.height);
                println!("Ribbon: {ribbon_length}");
                ribbon_length
            })
            .sum();

        Ok(ribbon_total)
    }
}
