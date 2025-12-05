use std::time::Instant;

const _EXAMPLE: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (fresh, available) = input.trim().split_once("\n\n").unwrap();

    let fresh = fresh
        .lines()
        .map(|line| {
            let (lo, hi) = line.split_once("-").unwrap();
            (lo.parse().unwrap(), hi.parse().unwrap())
        })
        .collect();

    let available = available.lines().map(|id| id.parse().unwrap()).collect();

    (fresh, available)
}

fn merge_ranges(mut fresh: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    fresh.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for range in fresh {
        if let Some(merged_range) = merged.last_mut()
            && range.0 <= merged_range.1 + 1
        {
            merged_range.1 = merged_range.1.max(range.1);
        } else {
            merged.push(range);
        }
    }
    merged
}

fn part1(fresh: &[(u64, u64)], available: &[u64]) -> usize {
    available
        .iter()
        .filter(|id| fresh.iter().any(|(lo, hi)| lo <= *id && *id <= hi))
        .count()
}

fn part2(fresh: &[(u64, u64)]) -> u64 {
    fresh.iter().map(|(lo, hi)| hi - lo + 1).sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day05.txt").unwrap();
    let (fresh, available) = parse(&input);
    let fresh = merge_ranges(fresh);

    let now = Instant::now();
    let result1 = part1(&fresh, &available);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&fresh);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
