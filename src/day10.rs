use itertools::Itertools;

lazy_static! {
    static ref NUMBERS: Vec<u64> = crate::read_lines("inputs/input_day10.txt")
        .unwrap()
        .map(|l| { l.unwrap().parse::<u64>().unwrap() })
        .collect_vec();
}

pub fn step1() -> u64 {
    let sorted = NUMBERS.iter().sorted().collect_vec();
    let ones = 1 + sorted.windows(2).filter(|l| l[0] + 1 == *l[1]).count() as u64;
    let threes = 1 + sorted.windows(2).filter(|l| l[0] + 3 == *l[1]).count() as u64;
    ones * threes
}

pub fn step2() -> u64 {
    let sorted = NUMBERS.iter().sorted().collect_vec();
    let max = 3 + **sorted.iter().max().unwrap();
    let extended = [vec![&0u64], sorted, vec![&max]].concat();

    let indexes = extended
        .windows(2)
        .enumerate()
        .filter(|(_, l)| l[0] + 3 == *l[1])
        .map(|(i, _)| i + 1)
        .collect_vec();

    [vec![0usize], indexes]
        .concat()
        .windows(2)
        .map(|i| {
            let vec = &extended[i[0]..i[1]];
            //println!("{:?} {:?}", vec, vec.len());
            match vec.len() {
                5 => 7,
                4 => 4,
                3 => 2,
                _ => 1,
            }
        })
        .product::<u64>()
}
