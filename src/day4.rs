use itertools::Itertools;
use std::io;

pub fn aoc(innerstep: fn(passports: Vec<(&str, &str)>) -> usize) -> io::Result<usize> {
    let lines = crate::read_lines("inputs/input_day4.txt")?;
    Ok(lines
        .map(|l| l.unwrap())
        .join(" ")
        .split("  ")
        .map(|l| {
            let passports = l
                .split_whitespace()
                .map(|p| p.split(':').collect_tuple().unwrap());
            let passports_without_cid = passports.filter(|(key, _val)| &"cid" != key).collect_vec();
            innerstep(passports_without_cid)
        })
        .sum())
}

pub fn step1(passports: Vec<(&str, &str)>) -> usize {
    let passport_size = passports.into_iter().count();
    (passport_size == 7).into()
}

pub fn step2(passports: Vec<(&str, &str)>) -> usize {
    let valid_passports = passports
        .into_iter()
        .filter(|(key, val)| match key as &str {
            "byr" => {
                let byr = val.parse::<usize>().unwrap();
                byr >= 1920 && byr <= 2002
            }
            "iyr" => {
                let iyr = val.parse::<usize>().unwrap();
                iyr >= 2010 && iyr <= 2020
            }
            "eyr" => {
                let eyr = val.parse::<usize>().unwrap();
                eyr >= 2020 && eyr <= 2030
            }
            "hgt" => {
                if val.ends_with("cm") {
                    let hgt = val.trim_end_matches("cm").parse::<usize>().unwrap();
                    hgt >= 150 && hgt <= 193
                } else if val.ends_with("in") {
                    let hgt = val.trim_end_matches("in").parse::<usize>().unwrap();
                    hgt >= 59 && hgt <= 76
                } else {
                    false
                }
            }
            "hcl" => {
                let color = val.replace("#", "");
                val.len() == 7 && color.len() == 6 && usize::from_str_radix(&color, 16).is_ok()
            }
            "ecl" => { ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] }.contains(val),
            "pid" => val.len() == 9 && val.chars().all(char::is_numeric),
            _ => false,
        });
    let passport_size = valid_passports.into_iter().count();
    (passport_size == 7).into()
}
