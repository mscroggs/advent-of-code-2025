use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
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

    #[cfg(feature = "test_input")]
    assert_eq!(result, 50);

    println!("{result}");
}
