use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut first = true;
    let mut ranges = vec![];
    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            first = false;
        } else if first {
            let mut parts = line.split("-");
            let start = parts.next().unwrap().parse::<i64>().unwrap();
            let end = parts.next().unwrap().parse::<i64>().unwrap();
            ranges.push((start, end));
        } else {
            let n = line.parse::<i64>().unwrap();
            for (start, end) in &ranges {
                if *start < n && n <= *end {
                    result += 1;
                    break;
                }
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 3);

    println!("{result}");
}
