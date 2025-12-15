use bitmask::BitMask;
use matrix::Matrix;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    iter::{once, repeat_n},
    time::Instant,
};

const _EXAMPLE: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[derive(Debug)]
struct Machine {
    indicator_diagram: BitMask,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i16>,
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (indicator_diagram, rest) = line.split_once(" ").unwrap();
            let indicator_diagram = BitMask::with_bits(
                indicator_diagram[1..indicator_diagram.len() - 1]
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(|(i, _)| i),
            );
            let (buttons, joltage) = rest.rsplit_once(" ").unwrap();
            let buttons = buttons
                .split(" ")
                .map(|btn| (btn[1..btn.len() - 1].split(",").map(|d| d.parse().unwrap())).collect())
                .collect();

            let joltage = joltage[1..joltage.len() - 1]
                .split(",")
                .map(|d| d.parse().unwrap())
                .collect();

            Machine {
                indicator_diagram,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn part1(machines: &[Machine]) -> usize {
    let mut presses = 0;
    for machine in machines {
        let btn_masks: Vec<_> = machine
            .buttons
            .iter()
            .map(|b| BitMask::with_bits(b.iter().copied()))
            .collect();

        let mut visited = HashSet::new();

        let mut queue: VecDeque<_> = btn_masks.iter().map(|b| (BitMask::new(), 0, b)).collect();

        while let Some((mut indicator, mut steps, btn)) = queue.pop_front() {
            indicator.toggle(*btn);
            if visited.contains(&indicator) {
                continue;
            }

            steps += 1;
            visited.insert(indicator);

            if indicator == machine.indicator_diagram {
                presses += steps;
                break;
            }
            queue.extend(btn_masks.iter().map(|b| (indicator, steps, b)));
        }
    }
    presses
}

fn to_tableau(augmented_matrix: &mut Matrix, objective_coeff: impl Iterator<Item = f64>) {
    let objective = objective_coeff.chain(once(0.0));
    augmented_matrix.insert_row(0, objective);
    let first_col = once(1.0).chain(repeat_n(0.0, augmented_matrix.height - 1));
    augmented_matrix.insert_column(0, first_col);
}

fn simplex_phase1(mut tableau: Matrix) -> Matrix {
    let orig_height = tableau.height;
    let orig_width = tableau.width;
    for y in 1..tableau.height {
        tableau.insert_column(
            tableau.width - 1,
            (0..tableau.height).map(|x| if x == y { 1.0 } else { 0.0 }),
        );
    }
    to_tableau(
        &mut tableau,
        repeat_n(0.0, orig_width - 1).chain(repeat_n(-1.0, orig_height - 1)),
    );
    for y in 0..orig_height - 1 {
        for x in 0..tableau.width {
            tableau[(x, 0)] += tableau[(x, y + 2)];
        }
    }
    let mut result = simplex_solve(tableau);
    result.remove_row(0);
    result.remove_column(0);
    for _ in 1..result.height {
        result.remove_column(result.width - 2);
    }
    result
}

fn simplex_solve(mut tableau: Matrix) -> Matrix {
    loop {
        let objective_row = tableau
            .get_row(0)
            .iter()
            .enumerate()
            .take(tableau.width - 1)
            .skip(1);
        let mut pivot_col = objective_row.filter(|(_, x)| **x > 0.0);
        if let Some((pivot_col, _)) = pivot_col.next() {
            let col = tableau.get_column(pivot_col);
            let basic = tableau.get_column(tableau.width - 1);
            let (pivot_row, _) = col
                .iter()
                .zip(basic)
                .enumerate()
                .skip(1)
                .filter(|(_, (a, _))| **a > 0.0)
                .map(|(i, (a, b))| (i, b / a))
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap();
            let pivot = tableau[(pivot_col, pivot_row)];
            if pivot == 0.0 {
                continue;
            }

            for x in 0..tableau.width {
                tableau[(x, pivot_row)] /= pivot;
            }
            for y in 0..tableau.height {
                if y == pivot_row {
                    continue;
                }
                let factor = tableau[(pivot_col, y)];
                for x in 0..tableau.width {
                    tableau[(x, y)] -= factor * tableau[(x, pivot_row)];
                }
            }
        } else {
            break;
        }
    }
    tableau
}

fn simplex(tableau: Matrix) -> Matrix {
    let tableau = simplex_phase1(tableau);
    let mut tableau = simplex_solve(tableau);

    for x in 0..tableau.width {
        let mut is_identity = true;
        let mut identity_idx = None;
        for y in 0..tableau.height {
            let val = tableau[(x, y)];
            if val != 0.0 {
                if val == 1.0 && identity_idx.is_none() {
                    identity_idx = Some(y);
                } else {
                    is_identity = false;
                }
            }
        }
        if is_identity && let Some(identity_idx) = identity_idx {
            tableau.swap_rows(identity_idx, x.min(tableau.height - 1));
        }
    }

    tableau
}

const EPS: f64 = 1e-12;

fn solve(machine: &Machine) -> usize {
    let mut cols: Vec<_> = machine
        .buttons
        .iter()
        .map(|btn| {
            let mut x = Vec::new();
            for i in 0..machine.joltage.len() {
                x.push(if btn.contains(&i) { 1. } else { 0. });
            }
            x
        })
        .collect();
    let joltage: Vec<_> = machine.joltage.iter().map(|j| *j as f64).collect();
    cols.push(joltage);
    let mut matrix = Matrix::from_columns(&cols);
    to_tableau(&mut matrix, repeat_n(-1.0, machine.buttons.len()));

    let mut queue = VecDeque::from([matrix]);
    let mut best = None;
    let mut bounds = HashSet::new();

    while let Some(tableau) = queue.pop_front() {
        let mut tableau = simplex(tableau);
        let result = tableau[(tableau.width - 1, 0)];
        if let Some(best) = best
            && result >= best as f64
        {
            continue;
        }

        let results = tableau.get_column(tableau.width - 1);

        let mut fract: Vec<_> = results
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, x)| x.fract() > EPS && x.fract() < (1. - EPS))
            .collect();
        fract.sort_by(|(_, a), (_, b)| b.fract().total_cmp(&a.fract()));
        if let Some((i, x)) = fract.first() {
            let (down, up) = (x.floor(), x.ceil());
            if bounds.contains(&(*i, down as i32, up as i32)) {
                continue;
            }
            bounds.insert((*i, down as i32, up as i32));

            tableau.insert_column(tableau.width - 1, repeat_n(0.0, tableau.height));

            let mut down_tableau = tableau.clone();
            down_tableau.append_row(
                repeat_n(0.0, *i)
                    .chain(once(1.0))
                    .chain(repeat_n(0.0, tableau.width.saturating_sub(i + 3)))
                    .chain(once(1.0))
                    .chain(once(down)),
            );
            queue.push_back(down_tableau);

            let mut up_tableau = tableau.clone();
            up_tableau.append_row(
                repeat_n(0.0, *i)
                    .chain(once(1.0))
                    .chain(repeat_n(0.0, tableau.width.saturating_sub(i + 3)))
                    .chain(once(-1.0))
                    .chain(once(up)),
            );
            queue.push_back(up_tableau);
        } else {
            best = Some(result as usize);
        }
    }

    best.unwrap()
}

fn part2(machines: &[Machine]) -> usize {
    machines.iter().map(solve).sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    let machines = parse(&input);

    let now = Instant::now();
    let result1 = part1(&machines);
    let time1 = now.elapsed();
    println!("part1: {result1} after {time1:?}");

    let now = Instant::now();
    let result2 = part2(&machines);
    let time2 = now.elapsed();

    println!("part2: {result2} after {time2:?}");
}

mod bitmask {
    use std::{
        fmt::{Debug, Display},
        ops::BitXor,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BitMask(u16);

    impl BitMask {
        pub fn new() -> Self {
            Self(0)
        }

        pub fn with_bits(bits: impl Iterator<Item = usize>) -> Self {
            let mut mask = 0;
            for i in bits {
                mask |= 1 << i;
            }
            Self(mask)
        }

        pub fn toggle(&mut self, button: BitMask) {
            self.0 ^= button.0;
        }
    }

    impl Debug for BitMask {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{:0>16b}]", self.0)
        }
    }

    impl Display for BitMask {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{:0>16b}]", self.0)
        }
    }

    impl BitXor for BitMask {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            Self(self.0 ^ rhs.0)
        }
    }
}

mod matrix {
    use std::{
        fmt::Display,
        ops::{Index, IndexMut, Mul, Range},
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Matrix {
        data: Vec<f64>,
        pub width: usize,
        pub height: usize,
    }
    impl Matrix {
        pub fn new(width: usize, height: usize) -> Self {
            Self {
                data: vec![0.0; width * height],
                width,
                height,
            }
        }

        pub fn from_rows(rows: &[Vec<f64>]) -> Self {
            let width = rows[0].len();
            let height = rows.len();
            let mut data = Vec::with_capacity(width * height);
            for row in rows {
                data.extend_from_slice(row);
            }
            Self {
                data,
                width,
                height,
            }
        }

        pub fn from_columns(cols: &[Vec<f64>]) -> Self {
            let width = cols.len();
            let height = cols[0].len();
            let mut data = Vec::with_capacity(width * height);
            for y in 0..height {
                for col in cols {
                    data.push(col[y]);
                }
            }
            Self {
                data,
                width,
                height,
            }
        }

        pub fn insert_row(&mut self, i: usize, row: impl Iterator<Item = f64>) {
            assert_eq!(Some(self.width), row.size_hint().1);
            self.data.reserve(self.width);
            let idx = i * self.width;
            self.data.splice(idx..idx, row);
            self.height += 1;
            assert_eq!(self.data.len(), self.width * self.height);
        }

        pub fn append_row(&mut self, row: impl Iterator<Item = f64>) {
            self.insert_row(self.height, row);
        }

        pub fn insert_column(&mut self, i: usize, column: impl Iterator<Item = f64>) {
            assert_eq!(Some(self.height), column.size_hint().1);
            self.data.reserve(self.height);
            for (y, c) in column.enumerate() {
                self.data.insert(y * self.width + i + y, c);
            }
            self.width += 1;
            assert_eq!(self.data.len(), self.width * self.height);
        }

        pub fn get_row(&self, y: usize) -> &[f64] {
            &self.data[y * self.width..(y + 1) * self.width]
        }

        pub fn get_column(&self, x: usize) -> Vec<f64> {
            let mut col = Vec::with_capacity(self.height);
            for y in 0..self.height {
                col.push(self[(x, y)]);
            }
            col
        }

        pub fn remove_row(&mut self, y: usize) {
            self.data.drain(y * self.width..(y + 1) * self.width);
            self.height -= 1;
        }

        pub fn remove_column(&mut self, x: usize) {
            for y in 0..self.height {
                self.data.remove(y * self.width + x - y);
            }
            self.width -= 1;
        }

        pub fn swap_rows(&mut self, a: usize, b: usize) {
            let (a, b) = (a.min(b), a.max(b));
            if a == b || b >= self.height {
                return;
            }
            let (first, second) = self.data.split_at_mut(b * self.width);
            let a = &mut first[a * self.width..(a + 1) * self.width];
            let b = &mut second[..self.width];
            a.swap_with_slice(b);
        }
    }

    impl Index<(usize, usize)> for Matrix {
        type Output = f64;

        fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
            assert!(
                x < self.width && y < self.height,
                "indexing ({}, {}) matrix at ({x}, {y}):\n{self}",
                self.width,
                self.height
            );
            &self.data[x + self.width * y]
        }
    }

    impl IndexMut<(usize, usize)> for Matrix {
        fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
            assert!(
                x < self.width && y < self.height,
                "indexing ({}, {}) matrix at ({x}, {y})",
                self.width,
                self.height
            );
            &mut self.data[x + self.width * y]
        }
    }

    impl Index<(Range<usize>, usize)> for Matrix {
        type Output = [f64];

        fn index(&self, (x, y): (Range<usize>, usize)) -> &Self::Output {
            assert!(x.end < self.width && y < self.height);
            &self.data[self.width * y..self.width * (y + 1)][x]
        }
    }

    impl<'b> Mul<&'b Matrix> for &Matrix {
        type Output = Matrix;

        fn mul(self, rhs: &'b Matrix) -> Self::Output {
            assert_eq!(
                self.width, rhs.height,
                "cant multiply matrices: \n{self} \n {rhs}"
            );
            let mut result = Matrix::new(rhs.width, self.height);
            for y in 0..self.height {
                for x in 0..rhs.width {
                    let mut sum = 0.0;
                    for i in 0..self.width {
                        sum += self[(i, y)] * rhs[(x, i)];
                    }
                    result[(x, y)] = sum;
                }
            }
            result
        }
    }

    impl Mul<f64> for &Matrix {
        type Output = Matrix;

        fn mul(self, rhs: f64) -> Self::Output {
            Self::Output {
                width: self.width,
                height: self.height,
                data: self.data.iter().map(|x| x * rhs).collect(),
            }
        }
    }

    impl Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            for y in 0..self.height {
                if y != 0 {
                    write!(f, " ")?;
                }
                write!(f, "[")?;
                for x in 0..self.width {
                    write!(
                        f,
                        "{:>6.2}",
                        self[(x, y)],
                        // precision = (self[(x, y)].fract() * 100.).min(2.) as usize
                    )?;
                    if x != self.width - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
                if y != self.height - 1 {
                    writeln!(f, ",")?;
                }
            }
            write!(f, "]")?;
            Ok(())
        }
    }
}
