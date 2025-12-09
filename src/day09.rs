use rect::Rectangle;
use std::time::Instant;

const _EXAMPLE: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

const _EXAMPLE2: &str = r"0,0
9,0
9,9
8,9
8,1
0,1
";

const _EXAMPLE3: &str = r"0,0
9,0
9,1
0,1
0,2
9,2
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

fn parse(input: &str) -> Vec<Position> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut it = line.split(",").map(|d| d.parse().unwrap());
            Position(it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn part1(tiles: &[Position]) -> usize {
    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| tiles.iter().skip(i + 1).map(|t2| (*t1, t2)))
        .map(|(t1, t2)| (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1))
        .max()
        .unwrap()
}

fn part2(tiles: &[Position]) -> usize {
    let mut edges = Vec::new();
    for i in 0..tiles.len() {
        let t1 = tiles[i];
        let t2 = tiles[(i + 1) % tiles.len()];

        let rect = Rectangle::new(
            (t1.0.min(t2.0), t1.1.min(t2.1)),
            (t1.0.max(t2.0), t1.1.max(t2.1)),
        );
        edges.push(rect);
    }

    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| tiles.iter().skip(i + 1).map(|t2| (*t1, t2)))
        .map(|(t1, t2)| {
            Rectangle::new(
                (t1.0.min(t2.0), t1.1.min(t2.1)),
                (t1.0.max(t2.0), t1.1.max(t2.1)),
            )
        })
        .filter(|rect| {
            let inner = Rectangle::new(
                (
                    (rect.left() + 1).min(rect.right()),
                    (rect.top() + 1).min(rect.bottom()),
                ),
                (
                    (rect.right() - 1).max(rect.left()),
                    (rect.bottom() - 1).max(rect.top()),
                ),
            );

            !edges.iter().any(|e| e.intersects(inner))
        })
        .map(|rect| rect.area())
        .max()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input/day09.txt").unwrap();
    let tiles = parse(&input);

    let now = Instant::now();
    let result1 = part1(&tiles);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&tiles);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}

mod rect {
    #![allow(dead_code)]

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Rectangle {
        top_left: (usize, usize),
        bottom_right: (usize, usize),
    }

    impl Rectangle {
        pub fn new(top_left: (usize, usize), bottom_right: (usize, usize)) -> Self {
            Self {
                top_left,
                bottom_right,
            }
        }

        pub fn top_left(&self) -> (usize, usize) {
            self.top_left
        }

        pub fn bottom_right(&self) -> (usize, usize) {
            self.bottom_right
        }

        pub fn top_right(&self) -> (usize, usize) {
            (self.bottom_right.0, self.top_left.1)
        }

        pub fn bottom_left(&self) -> (usize, usize) {
            (self.top_left.0, self.bottom_right.1)
        }

        pub fn right(&self) -> usize {
            self.bottom_right.0
        }

        pub fn left(&self) -> usize {
            self.top_left.0
        }

        pub fn top(&self) -> usize {
            self.top_left.1
        }

        pub fn bottom(&self) -> usize {
            self.bottom_right.1
        }

        pub fn width(&self) -> usize {
            self.right() - self.left() + 1
        }

        pub fn height(&self) -> usize {
            self.bottom() - self.top() + 1
        }

        pub fn area(&self) -> usize {
            self.width() * self.height()
        }

        pub fn intersects(&self, other: Rectangle) -> bool {
            ((self.left() <= other.left() && other.left() <= self.right())
                || (other.left() <= self.left() && self.left() <= other.right()))
                && ((self.top() <= other.top() && other.top() <= self.bottom())
                    || (other.top() <= self.top() && self.top() <= other.bottom()))
        }
    }

    #[test]
    fn test() {
        let r = Rectangle::new((2, 2), (5, 5));
        assert!(r.intersects(Rectangle::new((3, 3), (4, 4))));
        assert!(r.intersects(Rectangle::new((1, 1), (3, 3))));
        assert!(r.intersects(Rectangle::new((3, 1), (4, 3))));
        assert!(r.intersects(Rectangle::new((4, 1), (6, 4))));
        assert!(r.intersects(Rectangle::new((4, 3), (6, 4))));
        assert!(r.intersects(Rectangle::new((4, 4), (7, 7))));
        assert!(r.intersects(Rectangle::new((3, 4), (4, 5))));
        assert!(r.intersects(Rectangle::new((0, 4), (3, 6))));
        assert!(r.intersects(Rectangle::new((0, 2), (2, 3))));
        assert!(r.intersects(Rectangle::new((3, 0), (3, 7))));
        assert!(!r.intersects(Rectangle::new((0, 1), (1, 1))));
        assert!(!r.intersects(Rectangle::new((6, 5), (7, 7))));
        assert!(!r.intersects(Rectangle::new((6, 2), (7, 5))));

        let line = Rectangle::new((0, 2), (5, 2));
        assert!(line.intersects(Rectangle::new((3, 0), (3, 5))));
        assert!(line.intersects(Rectangle::new((0, 0), (0, 5))));
        assert!(line.intersects(Rectangle::new((0, 2), (0, 2))));
        assert!(line.intersects(Rectangle::new((5, 2), (5, 3))));
    }
}
