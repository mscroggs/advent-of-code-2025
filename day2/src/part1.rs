use std::fs::File;
use std::io::{prelude::*, BufReader};

fn solve(filename: &str) -> u64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let content = reader.lines().next().unwrap().unwrap();

    let mut result = 0;
    for r in content.split(",") {
        let mut r = r.split("-");
        let start = r.next().unwrap().parse::<u64>().unwrap();
        let end = r.next().unwrap().parse::<u64>().unwrap();
        for n in start..=end {
            let nstr = n.to_string();
            let ndigits = nstr.len();
            if ndigits.is_multiple_of(2) && nstr[..ndigits / 2] == nstr[ndigits / 2..] {
                result += n;
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
    fn day2_part1_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 1227775554);
    }
}
