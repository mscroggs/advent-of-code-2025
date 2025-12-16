use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, prelude::*};
use itertools::izip;


fn apply_transformation(x0: usize, y0: usize, n: usize, points: &[(usize, usize)]) -> Vec<(usize, usize)> {
    match n {
        0 => points.iter().map(|(x, y)| (x0 + *x, y0 + *y)).collect::<Vec<_>>(),
        1 => points.iter().map(|(x, y)| (x0 + *y, y0 + *x)).collect::<Vec<_>>(),
        2 => points.iter().map(|(x, y)| (x0 + 2-x, y0 + *y)).collect::<Vec<_>>(),
        3 => points.iter().map(|(x, y)| (x0 + 2-y, y0 + *x)).collect::<Vec<_>>(),
        4 => points.iter().map(|(x, y)| (x0 + *x, y0 + 2-y)).collect::<Vec<_>>(),
        5 => points.iter().map(|(x, y)| (x0 + *y, y0 + 2-x)).collect::<Vec<_>>(),
        6 => points.iter().map(|(x, y)| (x0 + 2-x, y0 + 2-y)).collect::<Vec<_>>(),
        7 => points.iter().map(|(x, y)| (x0 + 2-y, y0 + 2-x)).collect::<Vec<_>>(),
        _ => { panic!("Invalid n: {n}"); },
    }
}

#[derive(Debug, PartialEq)]
enum Status {
    Continue,
    Done,
}

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

fn overlap(
    parts: &[Vec<(usize, usize)>], placed: &[Vec<(usize, usize, usize)>],
) -> bool {
    let mut full = vec![];
    for (i, p) in placed.iter().enumerate() {
        for (x, y, r) in p {
            for j in apply_transformation(*x, *y, *r, &parts[i]) {
                if full.contains(&j) {
                    return true;
                }
                full.push(j);
            }
        }
    }
    false
}

fn try_all_positions(
    parts: &[Vec<(usize, usize)>], target: &Target, placed: &[Vec<(usize, usize, usize)>],
) -> bool {
    if overlap(parts, placed) {
        return false;
    }
    for (i, (p, q)) in izip!(&target.counts, placed).enumerate() {
        if q.len() < *p {
            if i + 1 == parts.len() && q.len() > 0 {
                println!("{placed:?}");
            }
            for x in if q.len() > 0 { q[q.len() - 1].0 } else {0}..target.width - 2 {
                for y in if q.len() > 0 && q[q.len() - 1].0 == x { q[q.len() - 1].1} else { 0 }..target.height - 2 {
                    for r in if q.len() > 0 && q[q.len() - 1].0 == x && q[q.len() - 1].0 == y { q[q.len() - 1].2 + 1} else { 0 }..8 {
                        let mut placed2 = placed.to_vec();
                        placed2[i].push((x, y, r));
                        if try_all_positions(parts, target, &placed2) {
                            return true;
                        }
                    }
                }
            }
            return false;
        }
    }
    true
}

fn fits(parts: &[Vec<(usize, usize)>], target: &Target) -> bool {
    let volume = izip!(parts, &target.counts).map(|(p, n)| *n * p.len()).sum::<usize>();
    if volume > target.area() {
        false
    } else if target.counts.iter().sum::<usize>() * 9 <= target.area() {
        true
    } else {
        panic!();
        try_all_positions(&parts, &target, &vec![vec![]; parts.len()])
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
                if line.is_empty() { continue; }
                let width = line.split("x").next().unwrap().parse::<usize>().unwrap();
                let height = line.split(":").next().unwrap().split("x").nth(1).unwrap().parse::<usize>().unwrap();
                let counts = line.split(": ").nth(1).unwrap().split(" ").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
                
                let t = Target{width, height, counts};
                if fits(&parts, &t) {
                    result += 1;
                }
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
