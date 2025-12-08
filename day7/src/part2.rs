use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
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

    let result = beams.iter().sum::<usize>();

    #[cfg(feature = "test_input")]
    assert_eq!(result, 40);

    println!("{result}");
}
