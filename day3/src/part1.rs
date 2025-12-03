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
        let mut first_digit = (0, 0);
        for (i, c) in line[..line.len() - 1].chars().enumerate() {
            let c = c.to_string().parse::<i32>().unwrap();
            if c > first_digit.1 {
                first_digit = (i, c);
            }
        }
        let mut second_digit = 0;
        for c in line[first_digit.0 + 1..].chars() {
            let c = c.to_string().parse::<i32>().unwrap();
            if c > second_digit {
                second_digit = c;
            }
        }
        result += 10 * first_digit.1 + second_digit;
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 357);

    println!("{result}");
}
