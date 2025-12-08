use itertools::izip;
use std::fs::File;
use std::io::{BufReader, prelude::*};

#[derive(Debug)]
enum EdgeCase {
    NewPart,
    OnePartFirst(usize),
    OnePartSecond(usize),
    TwoParts(usize, usize),
    SamePart,
}

fn edge_case(parts: &[Vec<usize>], edge: (usize, usize)) -> EdgeCase {
    let mut part0 = None;
    let mut part1 = None;
    for (n, p) in parts.iter().enumerate() {
        if p.contains(&edge.0) && p.contains(&edge.1) {
            return EdgeCase::SamePart;
        }
        if p.contains(&edge.0) {
            part0 = Some(n);
        }
        if p.contains(&edge.1) {
            part1 = Some(n);
        }
    }
    if let Some(i) = part0 {
        if let Some(j) = part1 {
            EdgeCase::TwoParts(i, j)
        } else {
            EdgeCase::OnePartFirst(i)
        }
    } else if let Some(j) = part1 {
        EdgeCase::OnePartSecond(j)
    } else {
        EdgeCase::NewPart
    }
}

fn squared_distance(a: &[usize], b: &[usize]) -> usize {
    izip!(a, b).map(|(i, j)| (i - j).pow(2)).sum::<usize>()
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let boxes = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(",")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut pairs = vec![];
    for (i, a) in boxes.iter().enumerate() {
        for (j, b) in boxes.iter().take(i).enumerate() {
            pairs.push((i, j, squared_distance(a, b)));
        }
    }

    pairs.sort_by(|a, b| a.2.cmp(&b.2));

    let mut parts = vec![];
    let mut result = 0;

    for (i, j, _) in pairs.iter() {
        let ec = edge_case(&parts, (*i, *j));
        if let EdgeCase::SamePart = ec {
            /* do nothing */
        } else {
            result = boxes[*i][0] * boxes[*j][0];
            match ec {
                EdgeCase::NewPart => {
                    parts.push(vec![*i, *j]);
                }
                EdgeCase::OnePartFirst(a) => parts[a].push(*j),
                EdgeCase::OnePartSecond(a) => parts[a].push(*i),
                EdgeCase::TwoParts(a, b) => {
                    let p = parts[b].clone();
                    for i in p {
                        parts[a].push(i);
                    }
                    parts.remove(b);
                }
                _ => {
                    panic!();
                }
            }
            if parts[0].len() == boxes.len() {
                break;
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 25272);

    println!("{result}");
}
