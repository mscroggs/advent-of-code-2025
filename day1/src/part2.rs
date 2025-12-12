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
            if n == 0 {
                n += 100;
            }
            n -= distance;
        } else {
            n += distance;
        }
        while n < 0 {
            n += 100;
            result += 1;
        }
        while n > 100 {
            n -= 100;
            result += 1;
        }
        if n == 100 {
            n = 0;
            result += 1;
        } else if n == 0 {
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
    fn day1_part2_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 6);
    }
}
