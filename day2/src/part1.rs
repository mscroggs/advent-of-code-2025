use std::fs::File;
use std::io::{prelude::*, BufReader};

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
            let nstr = n.to_string();
            let ndigits = nstr.len();
            if ndigits.is_multiple_of(2) && nstr[..ndigits / 2] == nstr[ndigits / 2..] {
                result += n;
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 1227775554);

    println!("{result}");
}
