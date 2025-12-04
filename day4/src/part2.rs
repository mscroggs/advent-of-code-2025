use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    #[cfg(feature = "test_input")]
    let file = File::open("test_input").unwrap();
    #[cfg(not(feature = "test_input"))]
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut positions = reader
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;
    loop {
        let mut to_remove = vec![];
        for (i, row) in positions.iter().enumerate() {
            for (j, p) in row.iter().enumerate() {
                if *p
                    && [
                        (i - 1, j - 1),
                        (i - 1, j),
                        (i - 1, j + 1),
                        (i, j - 1),
                        (i, j + 1),
                        (i + 1, j - 1),
                        (i + 1, j),
                        (i + 1, j + 1),
                    ]
                    .iter()
                    .map(|(a, b)| {
                        if let Some(r) = positions.get(*a)
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
                    to_remove.push((i, j));
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }

        for (i, j) in to_remove {
            positions[i][j] = false;
            result += 1;
        }
    }

    #[cfg(feature = "test_input")]
    assert_eq!(result, 43);

    println!("{result}");
}
