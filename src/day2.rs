use itertools::Itertools;
use std::io;

pub fn aoc(
    innerstep: fn(password: &str, c: char, min: usize, max: usize) -> usize,
) -> io::Result<usize> {
    let lines = crate::read_lines("inputs/input_day2.txt")?;
    Ok(lines
        .map(|l| {
            let line = l.unwrap();
            let (min_max, b, password) = &line
                .split_whitespace()
                .collect_tuple::<(_, _, _)>()
                .unwrap();
            let (min, max) = min_max
                .split('-')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap();
            let c = b.chars().next().unwrap();
            innerstep(password, c, min, max)
        })
        .sum())
}

pub fn step1(password: &str, c: char, min: usize, max: usize) -> usize {
    let len = password.matches(c).collect::<String>().len();
    (len >= min && len <= max).into()
}

pub fn step2(password: &str, c: char, min: usize, max: usize) -> usize {
    let first_char = password.chars().nth(min - 1).unwrap();
    let second_char = password.chars().nth(max - 1).unwrap();
    ((first_char == c) ^ (second_char == c)).into()
}
