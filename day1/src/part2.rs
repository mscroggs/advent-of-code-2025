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
            if n == 0 {
                n += 100;
            }
            n -= distance;
        } else {
            n += distance;
        }
        println!("{n}");
        while n < 0 {
            n += 100;
            result += 1;
        }
        while n > 100 {
            n -= 100;
            result += 1;
        }
        if n == 100 {
            n = 0;
            result += 1;
        } else if n == 0 {
            result += 1;
        }
        println!("{line} -> {n}  {result}");
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 6);

    println!("{result}");
}
