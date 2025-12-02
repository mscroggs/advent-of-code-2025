use std::fs::File;
use std::io::{prelude::*, BufReader};

fn is_valid(n: &str) -> bool {
    let nstr = n.to_string();
    let ndigits = nstr.len();
    for i in 1..=ndigits / 2 {
        if ndigits.is_multiple_of(i) {
            let mut a = true;
            for j in 1..ndigits / i {
                if nstr[..i] != nstr[i * j..i * (j + 1)] {
                    a = false;
                    break;
                }
            }
            if a {
                return true;
            }
        }
    }
    false
}

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let content = reader.lines().next().unwrap().unwrap();

    let mut result = 0;
    for r in content.split(",") {
        let mut r = r.split("-");
        let start = r.next().unwrap().parse::<u64>().unwrap();
        let end = r.next().unwrap().parse::<u64>().unwrap();
        for n in start..=end {
            if is_valid(&format!("{n}")) {
                result += n;
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 4174379265);

    println!("{result}");
}
