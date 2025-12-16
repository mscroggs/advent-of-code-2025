use itertools::izip;
use std::fs;

#[derive(Debug)]
struct Target {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl Target {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

fn fits(parts: &[Vec<(usize, usize)>], target: &Target) -> bool {
    let volume = izip!(parts, &target.counts)
        .map(|(p, n)| *n * p.len())
        .sum::<usize>();
    if volume > target.area() {
        false
    } else if target.counts.iter().sum::<usize>() * 9 <= target.area() {
        true
    } else {
        unimplemented!();
    }
}

fn solve(filename: &str) -> usize {
    let content = fs::read_to_string(filename).unwrap();

    let mut parts = vec![];

    let mut result = 0;
    for part in content.split("\n\n") {
        if part.contains('#') {
            assert!(part.starts_with(&format!("{}:", parts.len())));
            let mut this_part = vec![];
            for (i, row) in part.split("\n").skip(1).enumerate() {
                for (j, c) in row.chars().enumerate() {
                    if c == '#' {
                        this_part.push((i, j));
                    }
                }
            }
            parts.push(this_part);
        } else {
            for line in part.split("\n") {
                if line.is_empty() {
                    continue;
                }
                let width = line.split("x").next().unwrap().parse::<usize>().unwrap();
                let height = line
                    .split(":")
                    .next()
                    .unwrap()
                    .split("x")
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let counts = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                let t = Target {
                    width,
                    height,
                    counts,
                };
                if fits(&parts, &t) {
                    result += 1;
                }
                println!("{result}");
            }
        }
    }

    result
}

fn main() {
    println!("{}", solve("input"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day12_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 2);
    }
}
