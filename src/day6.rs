use itertools::Itertools;
use std::collections::HashSet;
use std::io;

pub fn aoc(innerstep: fn(line: &str) -> usize) -> io::Result<usize> {
    let lines = crate::read_lines("inputs/input_day6.txt")?;
    Ok(lines
        .map(|l| l.unwrap())
        .join(" ")
        .split("  ")
        .map(|l| innerstep(l))
        .sum())
}

pub fn step1(line: &str) -> usize {
    line.replace(" ", "").chars().unique().count()
}

pub fn step2(line: &str) -> usize {
    let mut iter = line.split_whitespace();
    let mut acc: HashSet<_> = iter.next().unwrap().chars().sorted().collect();
    for line in iter {
        let set: HashSet<char> = line.chars().sorted().collect();
        acc = acc.intersection(&set).cloned().collect();
    }
    acc.len()
}
