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
            beams.push(0);
            match c {
                'S' => {
                    assert_eq!(splitters.len(), 0);
                    beams[col] += 1;
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

    for row in splitters {
        for s in row {
            beams[s + 1] += beams[s];
            beams[s - 1] += beams[s];
            beams[s] = 0;
        }
    }

    beams.iter().sum::<usize>()
}

fn main() {
    println!("{}", solve("input"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day7_part2_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 40);
    }
}
