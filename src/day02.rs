use std::time::Instant;

const _EXAMPLE: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(",")
        .map(|range| {
            let (left, right) = range.split_once("-").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

fn num_digits(x: u64) -> u32 {
    x.ilog10() + 1
}

fn has_pattern1(x: u64) -> bool {
    let num_digits = num_digits(x);
    if num_digits % 2 == 1 {
        return false;
    }

    let mid_factor = 10u64.pow(num_digits / 2);
    let (upper, lower) = (x / mid_factor, x % mid_factor);

    upper == lower
}

fn day01(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .flat_map(|(low, high)| (*low..=*high).filter(|x| has_pattern1(*x)))
        .sum()
}

struct Splitter {
    value: u64,
    split: u64,
}

impl Splitter {
    fn new(value: u64, split_every: u32) -> Self {
        Self {
            value,
            split: 10u64.pow(split_every),
        }
    }
}

impl Iterator for Splitter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            return None;
        }
        let (rest, split) = (self.value / self.split, self.value % self.split);
        self.value = rest;
        Some(split as u32)
    }
}

fn has_pattern2(x: u64) -> bool {
    let num_digits = num_digits(x);
    (1..=num_digits / 2).any(|split_every| {
        if !num_digits.is_multiple_of(split_every) {
            return false;
        }

        let mut splitter = Splitter::new(x, split_every);
        let first = splitter.next().unwrap();
        splitter.all(|x| first == x)
    })
}

fn day02(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .flat_map(|(low, high)| (*low..=*high).filter(|x| has_pattern2(*x)))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    let input = parse(&input);

    let now = Instant::now();
    let result1 = day01(&input);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = day02(&input);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
