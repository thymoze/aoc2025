use std::{array, time::Instant};

const _EXAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111
";

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn part1(banks: &[Vec<u8>]) -> u32 {
    banks
        .iter()
        .map(|batteries| {
            let length = batteries.len();
            batteries.iter().enumerate().fold(
                (u8::MIN, u8::MIN),
                |(max_hi, max_lo), (idx, &battery)| {
                    if idx != length - 1 && battery > max_hi {
                        (battery, u8::MIN)
                    } else if battery > max_lo {
                        (max_hi, battery)
                    } else {
                        (max_hi, max_lo)
                    }
                },
            )
        })
        .map(|(hi, lo)| hi as u32 * 10 + lo as u32)
        .sum()
}

fn part2(banks: &[Vec<u8>]) -> u64 {
    banks
        .iter()
        .map(|batteries| {
            let length = batteries.len();
            batteries.iter().enumerate().fold(
                array::from_fn::<_, 12, _>(|_| u8::MIN),
                |mut max, (idx, &battery)| {
                    let remaining = length - idx;
                    let start = max.len().saturating_sub(remaining);
                    for i in start..max.len() {
                        if battery > max[i] {
                            max[i] = battery;
                            for m in &mut max[i + 1..] {
                                *m = u8::MIN;
                            }
                            break;
                        }
                    }
                    max
                },
            )
        })
        .map(|max| {
            max.iter().enumerate().fold(0, |num, (idx, x)| {
                num + 10u64.pow((max.len() - 1 - idx) as u32) * *x as u64
            })
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day03.txt").unwrap();
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
