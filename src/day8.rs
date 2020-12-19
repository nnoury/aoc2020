use itertools::Itertools;
use std::io;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
    Hlt,
}

fn parse(filename: &str) -> io::Result<Vec<Opcode>> {
    let lines = crate::read_lines(filename)?;
    Ok(lines
        .filter_map(|l| {
            let line = l.unwrap();
            let (opcode, value) = line.split_whitespace().collect_tuple().unwrap();
            let val = value.parse::<i64>().unwrap();
            match opcode {
                "nop" => Some(Opcode::Nop(val)),
                "acc" => Some(Opcode::Acc(val)),
                "jmp" => Some(Opcode::Jmp(val)),
                _ => None,
            }
        })
        .collect_vec())
}

lazy_static! {
    static ref PROG: Vec<Opcode> = parse("inputs/input_day8.txt").unwrap();
}

fn end(prog: &Vec<Opcode>) -> (bool, i64) {
    let mut program = prog.clone();
    let mut acc = 0;
    let mut pos = 0;
    let len = program.len();
    let mut ended = false;
    loop {
        if pos >= len {
            ended = true;
            break;
        }
        //println!("{:?} {:?} {:?}", pos, acc, program[pos]);
        match program[pos] {
            Opcode::Nop(_) => {
                program[pos] = Opcode::Hlt;
                pos += 1;
            }
            Opcode::Acc(val) => {
                program[pos] = Opcode::Hlt;
                pos += 1;
                acc += val;
            }
            Opcode::Jmp(delta) => {
                program[pos] = Opcode::Hlt;
                pos = (pos as i64 + delta) as usize;
            }
            Opcode::Hlt => break,
        }
    }
    (ended, acc)
}

pub fn step1() -> io::Result<i64> {
    let (_, result) = end(&PROG);
    Ok(result)
}

pub fn step2() -> io::Result<i64> {
    for i in 0..PROG.len() {
        let mut current_prog = PROG.clone();
        current_prog[i] = match current_prog[i] {
            Opcode::Nop(val) => Opcode::Jmp(val),
            Opcode::Jmp(val) => Opcode::Nop(val),
            op => op,
        };
        let (ended, result) = end(&current_prog);
        if ended {
            return Ok(result);
        }
    }
    Err(Error::new(ErrorKind::NotFound, "no solution found"))
}
