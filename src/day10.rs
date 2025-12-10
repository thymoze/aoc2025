use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    ops::{Add, BitXor, Mul},
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
    joltage: JoltageLevels,
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

            let joltage = JoltageLevels(
                joltage[1..joltage.len() - 1]
                    .split(",")
                    .map(|d| d.parse().unwrap())
                    .collect(),
            );

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct JoltageLevels(Vec<u8>);

impl JoltageLevels {
    fn distance(&self, other: &JoltageLevels) -> u32 {
        self.0
            .iter()
            .zip(&other.0)
            .map(|(a, b)| a.wrapping_sub(*b) as u32)
            .fold(0, |acc, x| acc.saturating_add(x))
    }
}

impl<'b> Add<&'b JoltageLevels> for &JoltageLevels {
    type Output = JoltageLevels;

    fn add(self, rhs: &'b JoltageLevels) -> Self::Output {
        JoltageLevels(self.0.iter().zip(&rhs.0).map(|(a, b)| a + b).collect())
    }
}

impl Mul<u8> for &JoltageLevels {
    type Output = JoltageLevels;

    fn mul(self, rhs: u8) -> Self::Output {
        JoltageLevels(self.0.iter().map(|x| x * rhs).collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JoltageCounter {
    step: usize,
    joltage: JoltageLevels,
    target: JoltageLevels,
}

impl PartialOrd for JoltageCounter {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.target != other.target {
            return None;
        }
        Some(self.cmp(other))
    }
}

impl Ord for JoltageCounter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert!(self.target == other.target);
        self.target
            .distance(&other.joltage)
            .cmp(&self.target.distance(&self.joltage))
    }
}

fn part2(machines: &[Machine]) -> usize {
    let mut presses = 0;
    for (i, machine) in machines.iter().enumerate() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{i} / {}", machines.len());

        let btn_joltages: Vec<_> = machine
            .buttons
            .iter()
            .map(|btn| {
                let mut levels = vec![0; machine.joltage.0.len()];
                for &i in btn {
                    levels[i] = 1;
                }
                JoltageLevels(levels)
            })
            .collect();

        let mut visited: HashMap<JoltageLevels, usize> = HashMap::new();
        let mut queue = BinaryHeap::from_iter(btn_joltages.iter().map(|j| JoltageCounter {
            step: 1,
            joltage: j.clone(),
            target: machine.joltage.clone(),
        }));
        'outer: while let Some(counter) = queue.pop() {
            // let distance = machine.joltage.distance(&counter.joltage);
            // println!("{:?}", (&counter, distance));

            for joltage in &btn_joltages {
                for factor in [20u8, 10, 1] {
                    let next_step = counter.step + factor as usize;
                    let next = &counter.joltage + &(joltage * factor);
                    if next == machine.joltage {
                        presses += next_step;
                        break 'outer;
                    }

                    if let Some(step) = visited.get_mut(&next) {
                        if next_step < *step {
                            *step = next_step;
                            // *prev = counter.joltage.clone();
                        } else {
                            continue;
                        }
                    } else {
                        visited.insert(next.clone(), next_step);
                    }

                    queue.push(JoltageCounter {
                        step: next_step,
                        joltage: next,
                        target: counter.target.clone(),
                    });
                }
            }
        }
    }
    presses
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
