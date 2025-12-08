use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    time::Instant,
};

const _EXAMPLE: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

#[derive(Debug, Clone, Copy)]
struct Position(i64, i64, i64);

impl Position {
    fn distance(&self, other: Position) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt()
    }
}

fn parse(input: &str) -> Vec<Position> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut it = line.split(",").map(|d| d.parse().unwrap());
            Position(it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Connection {
    distance: f64,
    from: usize,
    to: usize,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Connection {}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.total_cmp(&self.distance)
    }
}

fn distances(junction_boxes: &[Position]) -> BinaryHeap<Connection> {
    let mut distances = BinaryHeap::new();
    for (i, a) in junction_boxes.iter().enumerate() {
        for (j, b) in junction_boxes[i + 1..].iter().enumerate() {
            let conn = Connection {
                distance: a.distance(*b),
                from: i,
                to: j + i + 1,
            };

            distances.push(conn);
        }
    }
    distances
}

fn part1(mut distances: BinaryHeap<Connection>, n: usize) -> usize {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..n {
        if let Some(min) = distances.pop() {
            let mut circuit = HashSet::from_iter([min.from, min.to]);
            for c in circuits.extract_if(.., |c| c.contains(&min.from) || c.contains(&min.to)) {
                circuit = &circuit | &c;
            }
            circuits.push(circuit);
        }
    }
    let mut sizes: Vec<_> = circuits.into_iter().map(|c| c.len()).collect();
    sizes.sort_by_key(|&x| Reverse(x));
    sizes.iter().take(3).product()
}

fn part2(junction_boxes: &[Position], mut distances: BinaryHeap<Connection>) -> i64 {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let conn = loop {
        if let Some(min) = distances.pop() {
            let mut circuit = HashSet::from_iter([min.from, min.to]);
            for c in circuits.extract_if(.., |c| c.contains(&min.from) || c.contains(&min.to)) {
                circuit = &circuit | &c;
            }
            if circuit.len() == junction_boxes.len() {
                break min;
            } else {
                circuits.push(circuit);
            }
        }
    };

    let box1 = junction_boxes[conn.from];
    let box2 = junction_boxes[conn.to];
    box1.0 * box2.0
}

fn main() {
    let input = std::fs::read_to_string("input/day08.txt").unwrap();
    let junction_boxes = parse(&input);
    let distances = distances(&junction_boxes);

    let now = Instant::now();
    let result1 = part1(distances.clone(), 1000);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&junction_boxes, distances);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
