use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn solve(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut machines = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(": ");
        let machine = String::from(parts.next().unwrap());
        let outputs = parts
            .next()
            .unwrap()
            .split(" ")
            .map(String::from)
            .collect::<Vec<_>>();
        machines.insert(machine, outputs);
    }

    let mut paths = vec![vec!["you"]];
    let mut valid_paths = vec![];

    loop {
        let mut new_paths = vec![];

        for p in paths {
            for next in &machines[p[p.len() - 1]] {
                let mut new_p = p.clone();
                new_p.push(next);
                if next == "out" {
                    valid_paths.push(new_p);
                } else {
                    new_paths.push(new_p);
                }
            }
        }

        if new_paths.len() == 0 {
            break;
        }
        paths = new_paths;
    }

    valid_paths.len()
}

fn main() {
    println!("{}", solve("input"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day11_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 5);
    }
}
