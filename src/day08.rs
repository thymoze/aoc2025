use std::{mem::swap, time::Instant};

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

struct DisjointSet {
    pub parents: Vec<usize>,
    pub sizes: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parents[root] != root {
            root = self.parents[root];
        }
        while self.parents[x] != x {
            let next = self.parents[x];
            self.parents[x] = root;
            x = next;
        }
        root
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.sizes[root]
    }

    fn union(&mut self, mut x: usize, mut y: usize) {
        x = self.find(x);
        y = self.find(y);

        if x == y {
            return;
        }

        if self.sizes[x] < self.sizes[y] {
            swap(&mut x, &mut y);
        }
        self.parents[y] = x;
        self.sizes[x] += self.sizes[y];
    }
}

fn distances(junction_boxes: &[Position]) -> Vec<Connection> {
    let mut distances = Vec::new();
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
    distances.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    distances
}

fn part1(junction_boxes: &[Position], distances: &[Connection], n: usize) -> usize {
    let mut circuits = DisjointSet::new(junction_boxes.len());
    for min in distances.iter().take(n) {
        circuits.union(min.from, min.to);
    }
    let mut sizes = circuits.sizes.clone();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

fn part2(junction_boxes: &[Position], distances: &[Connection]) -> i64 {
    let mut circuits = DisjointSet::new(junction_boxes.len());
    for min in distances {
        circuits.union(min.from, min.to);
        if circuits.size(min.from) == junction_boxes.len() {
            let box1 = junction_boxes[min.from];
            let box2 = junction_boxes[min.to];
            return box1.0 * box2.0;
        }
    }
    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("input/day08.txt").unwrap();
    let junction_boxes = parse(&input);
    let distances = distances(&junction_boxes);

    let now = Instant::now();
    let result1 = part1(&junction_boxes, &distances, 1000);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&junction_boxes, &distances);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
