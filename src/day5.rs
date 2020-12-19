use itertools::Itertools;
use std::io;
use std::io::{Error, ErrorKind};
use std::ops::Range;

fn innerstep() -> Vec<usize> {
    fn parse(seat: &str, range: Range<usize>, one: char, zero: char) -> usize {
        let binary_str = seat[range].replace(one, "1").replace(zero, "0");
        usize::from_str_radix(&binary_str, 2).unwrap()
    }
    let lines = crate::read_lines("inputs/input_day5.txt").unwrap();
    lines
        .map(|l| {
            let line = l.unwrap();
            let row = parse(&line, 0..7, 'B', 'F');
            let col = parse(&line, 7..10, 'R', 'L');
            row * 8 + col
        })
        .collect_vec()
}

pub fn step1() -> io::Result<usize> {
    innerstep()
        .into_iter()
        .max()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "error"))
}

pub fn step2() -> io::Result<usize> {
    let mut previous: usize = 0;
    for id in innerstep().into_iter().sorted() {
        if id == previous + 2 {
            return Ok(previous + 1);
        }
        previous = id;
    }
    Err(Error::new(ErrorKind::InvalidData, "error"))
}
