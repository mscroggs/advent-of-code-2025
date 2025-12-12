use std::fs::File;
use std::io::{BufReader, prelude::*};

fn solve(filename: &str) -> usize {
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

    let mut result = 0;
    for (i, a) in tiles.iter().enumerate() {
        for b in tiles.iter().take(i) {
            let area = (1 + a[0].abs_diff(b[0])) * (1 + a[1].abs_diff(b[1]));
            if area > result {
                result = area;
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
    fn day9_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 50);
    }
}
