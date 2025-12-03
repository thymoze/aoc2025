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

fn part1(input: &Vec<i32>) -> u32 {
    let mut password = 0;
    let mut dial = 50;
    for x in input {
        dial = (dial + x).rem_euclid(100);
        password += u32::from(dial == 0);
    }
    password
}

fn part2(input: &Vec<i32>) -> u32 {
    let mut password = 0;
    let mut dial = 50;
    for x in input {
        let next = dial + x;
        let rotations = (next / 100).unsigned_abs();
        let crosses_zero = next == 0 || dial.signum() + next.signum() == 0;
        password += rotations + u32::from(crosses_zero);
        dial = next.rem_euclid(100);
    }
    password
}

fn main() {
    let input = std::fs::read_to_string("input/day01.txt").unwrap();
    let input = parse(&input);

    let now = Instant::now();
    let result1 = part1(&input);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&input);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
