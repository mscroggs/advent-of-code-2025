use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut ranges = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let mut parts = line.split("-");
        let start = parts.next().unwrap().parse::<i64>().unwrap();
        let end = parts.next().unwrap().parse::<i64>().unwrap();
        ranges.push((start, end));
    }
    ranges.sort();

    let mut result = 0;
    for (i, (start, end)) in ranges.iter().enumerate() {
        if i > 0 && *end <= ranges[i - 1].1 {
            continue;
        }
        if i > 0 && ranges[i - 1].1 >= *start {
            result += end - ranges[i - 1].1
        } else {
            result += end + 1 - start;
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 14);

    println!("{result}");
}
