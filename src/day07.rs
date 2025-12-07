use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const _EXAMPLE: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn parse(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let diagram: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut x = None;
    let y = diagram.iter().position(|line| {
        x = line.iter().position(|c| *c == 'S');
        x.is_some()
    });

    ((x.unwrap(), y.unwrap()), diagram)
}

fn spawn_tachyon(
    (x, mut y): (usize, usize),
    diagram: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) {
    while y < diagram.len() && diagram[y][x] != '^' {
        y += 1;
    }
    if y == diagram.len() {
        return;
    }
    if visited.insert((x, y)) {
        if x > 0 {
            spawn_tachyon((x - 1, y), diagram, visited);
        }
        if x < diagram[x].len() - 1 {
            spawn_tachyon((x + 1, y), diagram, visited);
        }
    }
}

fn part1(start: (usize, usize), diagram: &[Vec<char>]) -> u64 {
    let mut visited = HashSet::new();
    spawn_tachyon(start, diagram, &mut visited);
    visited.len() as u64
}

fn spawn_quantum_tachyon(
    (x, mut y): (usize, usize),
    diagram: &[Vec<char>],
    visited: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    while y < diagram.len() && diagram[y][x] != '^' {
        y += 1;
    }
    if y == diagram.len() {
        return 1;
    }
    if let Some(paths) = visited.get(&(x, y)) {
        *paths
    } else {
        let mut paths = 0;
        if x > 0 {
            paths += spawn_quantum_tachyon((x - 1, y), diagram, visited);
        }
        if x < diagram[x].len() - 1 {
            paths += spawn_quantum_tachyon((x + 1, y), diagram, visited);
        }
        visited.insert((x, y), paths);
        paths
    }
}

fn part2(start: (usize, usize), diagram: &[Vec<char>]) -> u64 {
    let mut visited = HashMap::new();
    spawn_quantum_tachyon(start, diagram, &mut visited)
}

fn main() {
    let input = std::fs::read_to_string("input/day07.txt").unwrap();
    let (start, diagram) = parse(&input);

    let now = Instant::now();
    let result1 = part1(start, &diagram);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(start, &diagram);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
