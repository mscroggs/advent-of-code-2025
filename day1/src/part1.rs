use std::fs::File;
use std::io::{prelude::*, BufReader};

fn solve(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut n = 50;
    let mut result = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let distance = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') {
            n -= distance;
        } else {
            n += distance;
        }
        n %= 100;
        if n == 0 {
            result += 1;
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
    fn day1_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 3);
    }
}
