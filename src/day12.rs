use std::{collections::HashSet, fmt::Display, ops::Index, time::Instant};

const _EXAMPLE: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    index: usize,
    width: usize,
    height: usize,
    pattern: Vec<bool>,
}
impl Shape {
    fn variants(&self) -> HashSet<Shape> {
        let r1 = self.rotate90cw();
        let r2 = r1.rotate90cw();
        let r3 = r2.rotate90cw();

        HashSet::from([
            self.clone(),
            r1.fliph(),
            r1.flipv(),
            r1,
            r2.fliph(),
            r2.flipv(),
            r2,
            r3.fliph(),
            r3.flipv(),
            r3,
        ])
    }

    fn rotate90cw(&self) -> Self {
        let mut rotated = vec![false; self.pattern.len()];
        for i in 0..self.width {
            for j in 0..self.height {
                rotated[i * self.height + (self.height - 1 - j)] = self.pattern[j * self.width + i];
            }
        }

        Self {
            index: self.index,
            width: self.height,
            height: self.width,
            pattern: rotated,
        }
    }

    fn fliph(&self) -> Self {
        let flipped = self
            .pattern
            .chunks(self.width)
            .flat_map(|chunk| chunk.iter().rev())
            .copied()
            .collect();
        Self {
            index: self.index,
            width: self.width,
            height: self.height,
            pattern: flipped,
        }
    }

    fn flipv(&self) -> Self {
        let flipped = self
            .pattern
            .chunks(self.width)
            .rev()
            .flatten()
            .copied()
            .collect();
        Self {
            index: self.index,
            width: self.width,
            height: self.height,
            pattern: flipped,
        }
    }

    fn width_at(&self, y: usize) -> usize {
        self.pattern[y * self.width..(y + 1) * self.width]
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(i, _)| i)
            .max()
            .unwrap_or(0)
    }

    fn area(&self) -> usize {
        self.pattern.iter().filter(|x| **x).count()
    }
}
impl Index<(usize, usize)> for Shape {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pattern[y * self.width + x]
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            writeln!(
                f,
                "{}",
                self.pattern[y * self.width..(y + 1) * self.width]
                    .iter()
                    .map(|x| if *x { '#' } else { '.' })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    quantities: [usize; 6],
    area: Vec<Vec<bool>>,
}
impl Region {
    fn try_place(&mut self, shape: &Shape) -> Result<(), ()> {
        let top_width = shape.width_at(0);
        for y in 0..self.height {
            for x in 0..self.width - top_width {
                match self.try_place_at(&shape, (x, y)) {
                    Ok(_) => return Ok(()),
                    Err(_) => {}
                }
            }
        }
        Err(())
    }

    fn try_place_at(&mut self, shape: &Shape, (x, y): (usize, usize)) -> Result<(), ()> {
        for j in 0..shape.height {
            for i in 0..shape.width {
                match self.area.get(y + j).and_then(|r| r.get(x + i)) {
                    Some(space) if *space && shape[(i, j)] => return Err(()),
                    None => return Err(()),
                    _ => {}
                }
            }
        }
        for j in 0..shape.height {
            for i in 0..shape.width {
                self.area[y + j][x + i] |= shape[(i, j)];
            }
        }
        self.quantities[shape.index] -= 1;
        Ok(())
    }
}
impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}x{}: {:?}", self.width, self.height, self.quantities)?;
        for row in &self.area {
            for x in row {
                write!(f, "{}", if *x { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let (shapes, regions) = input.trim().rsplit_once("\n\n").unwrap();
    let shapes = shapes
        .split("\n\n")
        .enumerate()
        .map(|(index, shape)| {
            let lines: Vec<_> = shape.lines().skip(1).collect();

            let pattern = shape
                .lines()
                .skip(1)
                .flat_map(|line| line.chars().map(|c| c == '#'))
                .collect();

            Shape {
                index,
                width: lines[0].len(),
                height: lines.len(),
                pattern,
            }
        })
        .collect();

    let regions = regions
        .lines()
        .map(|line| {
            let (size, quantities) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once("x").unwrap();
            let quantities = quantities
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let width = width.parse().unwrap();
            let height = height.parse().unwrap();
            Region {
                width,
                height,
                quantities,
                area: vec![vec![false; width]; height],
            }
        })
        .collect();

    (shapes, regions)
}

fn place_in_region(region: &mut Region, i: usize, shapes: &[Shape]) -> Result<(), ()> {
    if region.quantities[i] == 0 {
        if i + 1 < region.quantities.len() {
            return place_in_region(region, i + 1, shapes);
        } else {
            return Ok(());
        }
    }

    let mut reg = region.clone();
    let shape = &shapes[i];
    for shape in shape.variants() {
        match reg.try_place(&shape) {
            Ok(_) => {
                if let Err(_) = place_in_region(&mut reg, i, shapes) {
                    reg = region.clone();
                    continue;
                }

                *region = reg;
                return Ok(());
            }
            Err(_) => {}
        }
    }
    Err(())
}

fn part1(shapes: &[Shape], mut regions: Vec<Region>) -> usize {
    let mut feasible = 0;
    for region in &mut regions {
        let required: usize = region
            .quantities
            .iter()
            .enumerate()
            .map(|(i, q)| shapes[i].area() * q)
            .sum();
        let available = region.width * region.height;
        if available < required {
            continue;
        }
        if let Ok(_) = place_in_region(region, 0, shapes) {
            feasible += 1;
        }
    }
    feasible
}

fn main() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    let (shapes, regions) = parse(&input);

    let now = Instant::now();
    let result1 = part1(&shapes, regions.clone());
    let time1 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
}
