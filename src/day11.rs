use std::{
    collections::{HashMap, HashSet},
    iter,
    time::Instant,
};

const _EXAMPLE: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const _EXAMPLE2: &str = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (device, outputs) = line.split_once(":").unwrap();
            let outputs = outputs.trim().split(" ").map(|o| o.to_owned()).collect();
            (device.to_owned(), outputs)
        })
        .collect()
}

fn part1(devices: &HashMap<String, HashSet<String>>) -> usize {
    let mut queue = vec!["you".to_owned()];
    let mut paths = 0;
    while let Some(device) = queue.pop() {
        if device == "out" {
            paths += 1;
            continue;
        }
        let next = devices.get(&device).into_iter().flatten().cloned();
        queue.extend(next);
    }
    paths
}

fn part2(devices: &HashMap<String, HashSet<String>>) -> usize {
    let keys: Vec<_> = devices.keys().collect();
    let mut len = 0;
    let mut simplified: HashMap<String, HashMap<String, usize>> = devices
        .iter()
        .map(|(device, outputs)| {
            (
                device.clone(),
                outputs.iter().map(|o| (o.clone(), 1)).collect(),
            )
        })
        .collect();
    loop {
        for device in &keys {
            let mut combined = HashMap::new();
            for (output, n) in &simplified[*device] {
                if output == "out" || output == "fft" || output == "dac" {
                    combined
                        .entry(output.clone())
                        .and_modify(|x| *x += n)
                        .or_insert(*n);
                    continue;
                }

                for (k, v) in &simplified[output] {
                    combined
                        .entry(k.clone())
                        .and_modify(|x| *x += *v * n)
                        .or_insert(*v * n);
                }
            }
            if let Some(map) = simplified.get_mut(*device) {
                *map = combined;
            }
        }
        let new_len = simplified["svr"].len();
        if len == new_len {
            break;
        }
        len = new_len;
    }

    let mut result = 0;

    result += simplified["svr"].get("dac").unwrap_or(&0)
        * simplified["dac"].get("fft").unwrap_or(&0)
        * simplified["fft"].get("out").unwrap_or(&0);

    result += simplified["svr"].get("fft").unwrap_or(&0)
        * simplified["fft"].get("dac").unwrap_or(&0)
        * simplified["dac"].get("out").unwrap_or(&0);

    result
}

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    let devices = parse(&input);

    let now = Instant::now();
    let result1 = part1(&devices);
    let time1 = now.elapsed();

    let now = Instant::now();
    let result2 = part2(&devices);
    let time2 = now.elapsed();

    println!("part1: {result1} after {time1:?}");
    println!("part2: {result2} after {time2:?}");
}
