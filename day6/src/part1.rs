use std::fs::File;
use std::io::{BufReader, prelude::*};

fn solve(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut numbers = vec![];
    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("+") {
            for (i, op) in line.split(" ").filter(|n| !n.is_empty()).enumerate() {
                result += match op {
                    "+" => numbers.iter().map(|row: &Vec<i64>| row[i]).sum(),
                    "*" => {
                        let mut product = 1;
                        for row in &numbers {
                            product *= row[i];
                        }
                        product
                    }
                    _ => {
                        panic!("Invalid operation {op}");
                    }
                };
            }
        } else {
            numbers.push(
                line.split(" ")
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            );
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
    fn day6_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 4277556);
    }
}
