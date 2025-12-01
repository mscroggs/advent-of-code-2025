use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
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

    #[cfg(feature = "test_input")]
    assert_eq!(result, 3);

    println!("{result}");
}
