#![allow(unused_imports)]
use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;
use day_5::Day5;
use day_6::Day6;
use day_7::Day7;
use day_8::Day8;
use day_9::Day9;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

fn main() {
    // let day1_part1_result = Day1::part1();
    // let day1_part2_result = Day1::part2();
    // match day1_part2_result {
    //     Ok(value) => println!("RESULT: {value}"),
    //     Err(err) => eprintln!("Error: {}", err),
    // }
    // let day2_part1_result = Day2::part1();
    // println!("day2 p1: {:?}", day2_part1_result.unwrap());
    // let day2_part2 = Day2::part2();
    // println!("day22 p2: {:?}", day2_part2.unwrap());
    // let day3_part1 = Day3::part1();
    // println!("VISITED LOCATIONS: {}", day3_part1.unwrap().len());
    // let day3_part2 = Day3::part2();
    // println!("{}", day3_part2.unwrap());
    // let result = Day4::part2_parallel_v2("yzbqklnj".to_string());

    // let day5 = Day5::new();
    // let result = Day7::part2();
    let result = Day9::part2();

    println!("{:?}", result);
}
