use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let positions = reader
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;
    for (i, row) in positions.iter().enumerate() {
        for (j, p) in row.iter().enumerate() {
            if *p
                && [
                    if i > 0 && j > 0 {
                        Some((i - 1, j - 1))
                    } else {
                        None
                    },
                    if i > 0 { Some((i - 1, j)) } else { None },
                    if i > 0 { Some((i - 1, j + 1)) } else { None },
                    if j > 0 { Some((i, j - 1)) } else { None },
                    Some((i, j + 1)),
                    if j > 0 { Some((i + 1, j - 1)) } else { None },
                    Some((i + 1, j)),
                    Some((i + 1, j + 1)),
                ]
                .iter()
                .map(|c| {
                    if let Some((a, b)) = c
                        && let Some(r) = positions.get(*a)
                        && let Some(true) = r.get(*b)
                    {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
                    < 4
            {
                result += 1;
            }
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 13);

    println!("{result}");
}
