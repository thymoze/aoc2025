use std::time::Instant;

const _EXAMPLE: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

fn parse(input: &str) -> (&str, Vec<Op>) {
    let (number_lines, ops) = input.trim().rsplit_once("\n").unwrap();
    let ops = ops
        .split(" ")
        .filter(|op| !op.is_empty())
        .map(|op| match op {
            "*" => Op::Mul,
            "+" => Op::Add,
            _ => panic!("unknown operator"),
        })
        .collect();
    (number_lines, ops)
}

fn transpose<T>(v: impl Iterator<Item = impl Iterator<Item = T>>) -> Vec<Vec<T>> {
    let mut result = Vec::<Vec<T>>::new();
    for line in v {
        for (i, n) in line.enumerate() {
            if let Some(res) = result.get_mut(i) {
                res.push(n);
            } else {
                result.push(vec![n])
            }
        }
    }
    result
}

fn solve(problems: impl Iterator<Item = (Op, Vec<u64>)>) -> u64 {
    problems
        .map(|(op, operands)| match op {
            Op::Add => operands.iter().sum::<u64>(),
            Op::Mul => operands.iter().product(),
        })
        .sum()
}

fn part1(number_lines: &str, ops: &[Op]) -> u64 {
    let number_lines = number_lines.lines().map(|line| {
        line.split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
    });

    let numbers = transpose(number_lines);
    let problems = ops.iter().copied().zip(numbers);
    solve(problems)
}

fn part2(number_lines: &str, ops: &[Op]) -> u64 {
    let number_cols = transpose(number_lines.lines().map(|line| line.chars()));
    let mut numbers = Vec::new();
    let mut operands = Vec::new();
    for col in number_cols {
        let col: String = col.into_iter().collect::<String>();
        let col = col.trim();
        if col.is_empty() {
            numbers.push(operands);
            operands = Vec::new();
            continue;
        }
        let n: u64 = col.parse().unwrap();
        operands.push(n);
    }
    numbers.push(operands);
    let problems = ops.iter().copied().zip(numbers);
    solve(problems)
}

fn main() {
    let input = std::fs::read_to_string("input/day06.txt").unwrap();
    let (number_lines, ops) = parse(&input);

    let now = Instant::now();
    let result1 = part1(number_lines, &ops);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(number_lines, &ops);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
