use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};

#[derive(Debug)]
struct Machine {
    nbuttons: usize,
    target_on: Vec<usize>,
    wiring: Vec<Vec<usize>>,
    // joltage: Vec<usize>,
}

impl Machine {
    fn new(line: &str) -> Self {
        let mut nbuttons = 0;
        let mut target_on = vec![];
        let mut wiring = vec![];
        // let mut joltage = vec![];
        for part in line.split(" ") {
            if part.starts_with('[') {
                assert_eq!(nbuttons, 0);
                nbuttons = part.len() - 2;
                for (i, c) in part[1..part.len() - 1].chars().enumerate() {
                    if c == '#' {
                        target_on.push(i);
                    }
                }
            } else if part.starts_with('(') {
                wiring.push(
                    part[1..part.len() - 1]
                        .split(",")
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                );
            } else if part.starts_with('{') {
                // assert_eq!(joltage.len(), 0);
                // joltage = part[1..part.len() - 1].split(",").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
            } else {
                panic!("Invalid input");
            }
        }
        Self {
            nbuttons,
            target_on,
            wiring,
            // joltage,
        }
    }

    fn press(&self, n: usize, state: &[bool]) -> Vec<bool> {
        state
            .iter()
            .enumerate()
            .map(|(i, s)| if self.wiring[n].contains(&i) { !s } else { *s })
            .collect::<Vec<_>>()
    }

    fn nwires(&self) -> usize {
        self.wiring.len()
    }

    fn nbuttons(&self) -> usize {
        self.nbuttons
    }

    fn target(&self) -> Vec<bool> {
        (0..self.nbuttons)
            .map(|i| self.target_on.contains(&i))
            .collect::<Vec<_>>()
    }
}

fn solve(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut machines = vec![];
    for line in reader.lines() {
        machines.push(Machine::new(&line.unwrap()));
    }

    let mut result = 0;
    for m in &machines {
        let mut presses = HashMap::new();
        presses.insert(vec![false; m.nbuttons()], 0);
        let mut next_presses = vec![vec![false; m.nbuttons()]];
        let target = m.target();
        let mut np = 0;
        while !presses.contains_key(&target) {
            np += 1;
            let mut next_next_presses = vec![];
            for pos in &next_presses {
                for i in 0..m.nwires() {
                    let next = m.press(i, pos);
                    if !presses.contains_key(&next) {
                        presses.insert(next.clone(), np);
                        next_next_presses.push(next);
                    }
                }
            }
            next_presses = next_next_presses;
        }
        result += np;
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
    fn day10_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 7);
    }
}
