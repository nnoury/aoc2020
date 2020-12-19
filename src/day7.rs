extern crate regex;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io;

lazy_static! {
    static ref BAGS: HashMap<String, Vec<(u64, String)>> = aoc("inputs/input_day7.txt").unwrap();
}

fn aoc(filename: &str) -> io::Result<HashMap<String, Vec<(u64, String)>>> {
    let color_rg = Regex::new(r"(?P<color>.*) bags contain (?P<content>.*)\.").unwrap();
    let content_rg = Regex::new(r"(?P<number>\d+) (?P<color>.*) bags?").unwrap();

    let lines = crate::read_lines(filename)?;
    let bags_map: HashMap<String, Vec<(u64, String)>> = lines
        .map(|l| {
            let line = l.unwrap();
            let color_caps = color_rg.captures(&line).unwrap();
            let color = color_caps.name("color").unwrap().as_str().to_string();
            let content = color_caps
                .name("content")
                .unwrap()
                .as_str()
                .split(", ")
                .filter_map(|colored_bags| match content_rg.captures(&colored_bags) {
                    Some(content_caps) => Some((
                        content_caps
                            .name("number")
                            .unwrap()
                            .as_str()
                            .parse::<u64>()
                            .unwrap(),
                        content_caps.name("color").unwrap().as_str().to_string(),
                    )),
                    None => None,
                })
                .collect_vec();
            (color, content)
        })
        .collect();

    Ok(bags_map)
}

pub fn step1(color: &str) -> io::Result<u64> {
    //if contains add to results ; for each results add to result without repetition
    let mut size: u64;
    let mut result: HashSet<String> = HashSet::new();
    result.insert(color.into());

    loop {
        size = result.len() as u64;
        result.clone().into_iter().for_each(|color| {
            BAGS.clone().into_iter().for_each(|(key, val)| {
                if val.iter().any(|(_, col)| col == &color) {
                    result.insert(key);
                }
            });
        });

        if size == result.len() as u64 {
            break;
        }
    }
    Ok(result.len() as u64 - 1)
}

pub fn step2(color: &str) -> io::Result<u64> {
    let content = BAGS.get(color).unwrap();
    let sum: u64 = match content.len() {
        0 => 1,
        _ => {
            1 + content
                .into_iter()
                .map(|(size, name)| size * step2(name).unwrap())
                .sum::<u64>()
        }
    };
    Ok(sum)
}
