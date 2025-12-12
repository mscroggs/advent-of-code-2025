use std::fs::File;
use std::io::{BufReader, prelude::*};

fn solve(filename: &str) -> u64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| format!("{} ", line.unwrap()))
        .collect::<Vec<_>>();

    let operations = lines[lines.len() - 1]
        .split(" ")
        .filter(|n| !n.is_empty())
        .collect::<Vec<_>>();

    let mut op = 0;
    let mut numbers = vec![];
    let mut result = 0;

    for i in 0..lines[0].len() {
        let number = lines
            .iter()
            .take(lines.len() - 1)
            .map(|line| line.chars().nth(i).unwrap())
            .filter(|i| *i != ' ')
            .collect::<String>();
        if let Ok(n) = number.parse::<u64>() {
            numbers.push(n);
        } else {
            result += match operations[op] {
                "+" => numbers.iter().sum(),
                "*" => {
                    let mut product = 1;
                    for n in &numbers {
                        product *= n;
                    }
                    product
                }
                _ => {
                    panic!("Invalid operation {op}");
                }
            };
            op += 1;
            numbers = vec![];
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
    fn day6_part2_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 3263827);
    }
}
