use std::fs::File;
use std::io::{BufReader, prelude::*};

fn solve(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut beams = vec![];
    let mut splitters = vec![];

    for line in reader.lines() {
        let mut splitters_row = vec![];
        for (col, c) in line.unwrap().chars().enumerate() {
            match c {
                'S' => {
                    assert_eq!(splitters.len(), 0);
                    beams.push(col);
                }
                '^' => {
                    splitters_row.push(col);
                }
                '.' => {}
                _ => {
                    panic!("Invalid character {c}");
                }
            }
        }
        splitters.push(splitters_row);
    }

    let mut result = 0;

    for row in splitters {
        for s in row {
            if let Some(index) = beams.iter().position(|i| *i == s) {
                result += 1;
                beams.remove(index);
                if !beams.contains(&(s + 1)) {
                    beams.push(s + 1);
                }
                if !beams.contains(&(s - 1)) {
                    beams.push(s - 1);
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
    fn day7_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 21);
    }
}
