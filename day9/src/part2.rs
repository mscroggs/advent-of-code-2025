use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn contained(xmin: usize, xmax: usize, ymin: usize, ymax: usize, edges: &[Vec<usize>]) -> bool {
    for e in edges.iter().take(ymax).skip(ymin + 1) {
        for i in e {
            if xmin < *i && *i < xmax {
                return false;
            }
        }
    }
    true
}

fn solve(filename: &str) -> Option<usize> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let tiles = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(",")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut rectangles = vec![];
    for (i, a) in tiles.iter().enumerate() {
        for b in tiles.iter().take(i) {
            let area = (1 + a[0].abs_diff(b[0])) * (1 + a[1].abs_diff(b[1]));
            rectangles.push((
                min(a[0], b[0]),
                max(a[0], b[0]),
                min(a[1], b[1]),
                max(a[1], b[1]),
                area,
            ));
        }
    }

    rectangles.sort_by(|a, b| a.4.cmp(&b.4));
    rectangles.reverse();

    let mut edges = vec![vec![]; tiles.iter().map(|p| p[1]).max().unwrap() + 1];
    for (i, a) in tiles.iter().enumerate() {
        let b = &tiles[(i + 1) % tiles.len()];
        let xmin = min(a[0], b[0]);
        let xmax = max(a[0], b[0]);
        let ymin = min(a[1], b[1]);
        let ymax = max(a[1], b[1]);
        if xmin == xmax {
            for e in edges.iter_mut().take(ymax + 1).skip(ymin) {
                e.push(xmin);
            }
        } else {
            for i in xmin..=xmax {
                edges[ymin].push(i);
            }
        }
    }

    for (xmin, xmax, ymin, ymax, area) in rectangles {
        if contained(xmin, xmax, ymin, ymax, &edges) {
            return Some(area);
        }
    }
    None
}

fn main() {
    println!("{}", solve("input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day9_part1_test() {
        let solution = solve("test_input").unwrap();
        assert_eq!(solution, 24);
    }
}
