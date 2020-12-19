use itertools::Itertools;
use std::io;
use std::io::{Error, ErrorKind};

pub fn aoc(size: usize) -> io::Result<usize> {
    let lines = crate::read_lines("inputs/input_day1.txt")?;
    let nums = lines.map(|l| l.unwrap().parse::<usize>().unwrap());
    for perm in nums.combinations(size) {
        if perm.iter().sum::<usize>() == 2020 {
            return Ok(perm.iter().product::<usize>());
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "error"))
}
