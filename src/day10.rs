use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    ops::{Add, BitXor, Index, IndexMut, Mul, Range},
    slice::SliceIndex,
    time::Instant,
};

const _EXAMPLE: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BitMask(u16);

impl BitMask {
    fn new() -> Self {
        Self(0)
    }

    fn with_bits(bits: impl Iterator<Item = usize>) -> Self {
        let mut mask = 0;
        for i in bits {
            mask |= 1 << i;
        }
        Self(mask)
    }

    fn toggle(&mut self, button: BitMask) {
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

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    data: Vec<f32>,
    width: usize,
    height: usize,
}
impl Matrix {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0.0; width * height],
            width,
            height,
        }
    }

    fn from_rows(rows: &[Vec<f32>]) -> Self {
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

    fn from_columns(cols: &[Vec<f32>]) -> Self {
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

    fn get_column(&self, x: usize) -> Vec<f32> {
        let mut col = Vec::with_capacity(self.height);
        for y in 0..self.height {
            col.push(self[(x, y)]);
        }
        col
    }

    fn swap_rows(&mut self, a: usize, b: usize) {
        let (a, b) = (a.min(b), a.max(b));
        if a == b || b >= self.height {
            return;
        }
        let (first, second) = self.data.split_at_mut(b * self.width);
        let a = &mut first[a * self.width..(a + 1) * self.width];
        let b = &mut second[..self.width];
        a.swap_with_slice(b);
    }

    fn as_reduced_row_echelon_form(&mut self) {
        for k in 0..self.width - 1 {
            let x = k.min(self.width - 1);
            let y = k.min(self.height - 1);
            dbg!((x, y));
            println!("{self}\n");

            if self[(0..x, y)].iter().any(|v| *v != 0.0) {
                continue;
            }

            let mut val = self[(x, y)];
            let mut max_row = y;
            for j in (y + 1)..self.height {
                let v = self[(x, j)];
                if v > 0. && (val == 0.0 || v < val.abs()) {
                    val = self[(x, j)];
                    max_row = j;
                }
            }
            if y != max_row {
                // println!("swapping rows {y} and {max_row}");
                self.swap_rows(y, max_row);
            }

            if val == 0.0 {
                continue;
            }
            if val > 0.0 && val < 1.0 {
                // println!("normalizing current row: / {val}");
                for col in x..self.width {
                    self[(col, y)] /= val;
                }
                self[(x, y)] = 1.0;
            }

            for row in 0..self.height {
                if row == y {
                    continue;
                }
                let factor = self[(x, row)] / self[(x, y)];
                if factor == 0.0 {
                    continue;
                }
                // println!("row {row} -= {factor} * row {y}");
                for col in (x + 1)..self.width {
                    self[(col, row)] -= factor * self[(col, y)];
                }
                self[(x, row)] = 0.0;
            }
        }
    }
}
impl Index<(usize, usize)> for Matrix {
    type Output = f32;

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
    type Output = [f32];

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
                    "{:>4.precision$}",
                    self[(x, y)],
                    precision = (self[(x, y)].fract() * 100.).min(2.) as usize
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

fn solve_joltage(machine: &Machine) -> i16 {
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
    let btn_matrix = Matrix::from_columns(&cols);
    let joltage: Vec<_> = machine.joltage.iter().map(|j| *j as f32).collect();

    cols.push(joltage.clone());
    let mut matrix = Matrix::from_columns(&cols);
    // println!("{matrix}\n");
    matrix.as_reduced_row_echelon_form();

    // println!("{matrix}\n");
    let width = matrix.width;
    // for y in 0..matrix.height {
    //     if matrix[(width - 1, y)] < 0.0 {
    //         // for x in 0..width {
    //         matrix[(width - 1, y)] *= -1.0;
    //         // }
    //     }
    // }
    println!("{matrix}\n");

    let mut result = vec![0.0; machine.buttons.len()];
    let mut identity_columns = Vec::new();
    let mut lincomb_columns = Vec::new();
    for x in 0..width - 1 {
        let column = matrix.get_column(x);
        let identity_idx = x.min(matrix.height - 1);
        let mut is_identity_col = true;
        for y in 0..matrix.height {
            if y != identity_idx {
                is_identity_col = is_identity_col && matrix[(x, y)] == 0.0;
            } else {
                is_identity_col = is_identity_col && matrix[(x, y)] == 1.0;
            }
        }
        if is_identity_col && identity_columns.len() < matrix.height {
            identity_columns.push((x, identity_idx));
        } else {
            lincomb_columns.push((x, column.iter().sum::<f32>(), column));
        }
    }

    loop {
        let last = matrix.get_column(width - 1);
        let mut last_negative: Vec<_> =
            last.iter().enumerate().filter(|(_, x)| **x < 0.0).collect();
        if last_negative.is_empty() {
            break;
        }
        last_negative.sort_by(|a, b| a.1.total_cmp(b.1));
        dbg!(&last_negative);

        for (y, _) in last_negative {
            let mut max_negative = 0.0;
            let mut idx = None;
            for (i, (_, _, column)) in lincomb_columns.iter().enumerate() {
                let val = column[y];
                if val < max_negative {
                    let factor = matrix[(width - 1, y)] / column[y];
                    if factor.fract() != 0.0 {
                        continue;
                    }
                    // if last.iter().any(|x| (x / val).fract() != 0.0) {
                    //     continue;
                    // }

                    max_negative = val;
                    idx = Some(i);
                }
            }
            dbg!(y, max_negative, idx);
            if let Some(idx) = idx {
                let (x, _, column) = &lincomb_columns[idx];

                let mut factor = f32::MAX;
                for y in 0..matrix.height {
                    if column[y] > 0.0 || (column[y] < 0.0 && matrix[(width - 1, y)] < 0.0) {
                        let f = (matrix[(width - 1, y)] / column[y]).floor();
                        dbg!(y, f, matrix[(width - 1, y)]);
                        if f >= 0.0 {
                            factor = factor.min(f);
                        }
                    }
                }

                dbg!(y, x, factor);
                result[*x] += factor;
                for y in 0..matrix.height {
                    if column[y] != 0.0 {
                        matrix[(width - 1, y)] -= factor * column[y];
                    }
                }
            } else {
                panic!()
            }
            println!("{matrix}\n");
        }
    }

    // dbg!(&identity_columns, &lincomb_columns);

    lincomb_columns.sort_by(|a, b| b.1.total_cmp(&a.1));
    for (x, sum, column) in lincomb_columns {
        if sum > 1.0 {
            let mut min = f32::MAX;
            for y in 0..matrix.height {
                if column[y] > 0.0 {
                    min = min.min(matrix[(width - 1, y)] / column[y]);
                }
            }

            result[x] += min;
            for y in 0..matrix.height {
                if column[y] != 0.0 {
                    matrix[(width - 1, y)] -= min * column[y];
                }
            }
        } else {
            break;
        }
    }
    println!("{matrix}");
    dbg!(&result);
    for (x, i) in identity_columns {
        result[x] = matrix[(width - 1, i)];
    }
    dbg!(&result);
    // result[7] += 1.;

    let result_matrix = Matrix::from_columns(&[result.clone()]);

    let test = &btn_matrix * &result_matrix;
    let expected = Matrix::from_columns(std::slice::from_ref(&joltage));

    assert_eq!(
        test, expected,
        "multiplied \n{btn_matrix}\n * \n{result_matrix}\nfor machine\n{machine:?}\n"
    );

    result.iter().map(|r| *r as i16).sum()
}

fn part2(machines: &[Machine]) -> usize {
    dbg!(solve_joltage(&machines[11]));
    todo!();

    machines
        .iter()
        .map(|machine| solve_joltage(machine) as usize)
        .sum()
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
