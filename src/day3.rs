use std::io;

pub fn aoc(right: usize, down: usize) -> io::Result<usize> {
    let mut count = 0;
    let mut pos = 0;
    let lines = crate::read_lines("inputs/input_day3.txt")?;
    lines
        .enumerate()
        .filter(|(i, _)| i % down == 0)
        .for_each(|(_, l)| {
            if l.unwrap().chars().cycle().nth(pos).unwrap() == '#' {
                count += 1;
            }
            pos += right;
        });
    Ok(count)
}
