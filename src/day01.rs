use std::time::Instant;

const _EXAMPLE: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| match (&line[..1], &line[1..]) {
            ("R", num) => num.parse().unwrap(),
            ("L", num) => -num.parse::<i32>().unwrap(),
            _ => panic!("malformed input"),
        })
        .collect()
}

fn day01(input: &Vec<i32>) -> u32 {
    let mut count = 0;
    let mut sum = 50;
    for x in input {
        sum = (sum + x).rem_euclid(100);
        if sum == 0 {
            count += 1;
        }
    }
    count
}

fn day02(input: &Vec<i32>) -> u32 {
    let mut count = 0;
    let mut sum = 50;
    for x in input {
        let (div, rem) = (x / 100, x % 100);
        let next = sum + rem;
        count += div.unsigned_abs() + u32::from(sum != 0 && (next <= 0 || next >= 100));
        sum = next.rem_euclid(100);
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input/day01.txt").unwrap();
    let input = parse(&input);

    let result1 = day01(&input);
    let result2 = day02(&input);

    println!("part1: {result1}");
    println!("part2: {result2}");
}
