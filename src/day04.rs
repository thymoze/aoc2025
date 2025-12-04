use std::time::Instant;

const _EXAMPLE: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|x| x == '@').collect())
        .collect()
}

fn neighbors(rolls: &[Vec<bool>], (y, x): (usize, usize)) -> Vec<bool> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push(rolls[y][x - 1]);
    }
    if x < rolls[0].len() - 1 {
        neighbors.push(rolls[y][x + 1]);
    }
    if y > 0 {
        neighbors.push(rolls[y - 1][x]);
        if x > 0 {
            neighbors.push(rolls[y - 1][x - 1]);
        }
        if x < rolls[0].len() - 1 {
            neighbors.push(rolls[y - 1][x + 1]);
        }
    }
    if y < rolls.len() - 1 {
        neighbors.push(rolls[y + 1][x]);
        if x > 0 {
            neighbors.push(rolls[y + 1][x - 1]);
        }
        if x < rolls[0].len() - 1 {
            neighbors.push(rolls[y + 1][x + 1]);
        }
    }
    neighbors
}

fn part1(rolls: &[Vec<bool>]) -> u32 {
    let mut accessible = 0;
    for i in 0..rolls.len() {
        for j in 0..rolls[0].len() {
            if rolls[i][j] {
                let neighbors = neighbors(rolls, (i, j));
                if neighbors.iter().filter(|x| **x).count() < 4 {
                    accessible += 1;
                }
            }
        }
    }
    accessible
}

fn part2(mut rolls: Vec<Vec<bool>>) -> u64 {
    let mut accessible = 0;
    loop {
        let mut to_remove = Vec::new();
        for i in 0..rolls.len() {
            for j in 0..rolls[0].len() {
                if rolls[i][j] {
                    let neighbors = neighbors(&rolls, (i, j));
                    if neighbors.iter().filter(|x| **x).count() < 4 {
                        accessible += 1;
                        to_remove.push((i, j))
                    }
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }
        for (i, j) in to_remove {
            rolls[i][j] = false;
        }
    }
    accessible
}

fn main() {
    let input = std::fs::read_to_string("input/day04.txt").unwrap();
    let input = parse(&input);

    let now = Instant::now();
    let result1 = part1(&input);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(input);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
