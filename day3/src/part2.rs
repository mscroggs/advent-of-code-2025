use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let mut n = 0;
        let mut start = 0;
        for d in 0..12 {
            n *= 10;
            let mut next_digit = (0, 0);
            for (i, c) in line[start..line.len() - 11 + d].chars().enumerate() {
                let c = c.to_string().parse::<i64>().unwrap();
                if c > next_digit.1 {
                    next_digit = (i, c);
                }
            }
            n += next_digit.1;
            start += next_digit.0 + 1;
        }
        result += n;
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 3121910778619);

    println!("{result}");
}
