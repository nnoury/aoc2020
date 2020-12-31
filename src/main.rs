#[macro_use]
extern crate lazy_static;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    println!("day1 step1 {:?}", day1::aoc(2)?);
    println!("day1 step2 {:?}", day1::aoc(3)?);

    println!("day2 step1 {:?}", day2::aoc(day2::step1)?);
    println!("day2 step2 {:?}", day2::aoc(day2::step2)?);

    println!("day3 step1 {:?}", day3::aoc(3, 1)?);
    println!(
        "day3 step2 {:?}",
        day3::aoc(1, 1)?
            * day3::aoc(3, 1)?
            * day3::aoc(5, 1)?
            * day3::aoc(7, 1)?
            * day3::aoc(1, 2)?
    );

    println!("day4 step1 {:?}", day4::aoc(day4::step1)?);
    println!("day4 step2 {:?}", day4::aoc(day4::step2)?);

    println!("day5 step1 {:?}", day5::step1()?);
    println!("day5 step2 {:?}", day5::step2()?);

    println!("day6 step1 {:?}", day6::aoc(day6::step1)?);
    println!("day6 step2 {:?}", day6::aoc(day6::step2)?);

    println!("day7 step1 {:?}", day7::step1("shiny gold")?);
    println!("day7 step2 {:?}", day7::step2("shiny gold")? - 1);

    println!("day8 step1 {:?}", day8::step1()?);
    println!("day8 step2 {:?}", day8::step2()?);

    let day9step1 = day9::step1(25);
    println!("day9 step1 {:?}", day9step1);
    println!("day9 step2 {:?}", day9::step2(day9step1));

    println!("day10 step1 {:?}", day10::step1());
    println!("day10 step2 {:?}", day10::step2());
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
