use itertools::Itertools;

lazy_static! {
    static ref NUMBERS: Vec<u64> = crate::read_lines("inputs/input_day9.txt")
        .unwrap()
        .map(|l| { l.unwrap().parse::<u64>().unwrap() })
        .collect_vec();
}

pub fn step1(preamble: usize) -> u64 {
    for i in 0..NUMBERS.len() - preamble {
        let seeds = &NUMBERS[i..preamble + i];
        let sources = seeds.iter().tuple_combinations::<(_, _)>().collect_vec();
        let num = NUMBERS[i + preamble];
        if !sources.into_iter().any(|(a, b)| a + b == num) {
            return num;
        }
    }
    0
}

pub fn step2(num: u64) -> u64 {
    for i in 0..NUMBERS.len() {
        for j in i + 1..NUMBERS.len() {
            let vec = &NUMBERS[i..j];
            let sum: u64 = vec.iter().sum();
            if sum == num {
                return vec.iter().min().unwrap() + vec.iter().max().unwrap();
            }
        }
    }
    0
}
