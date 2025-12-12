use std::fs::File;
use std::io::{BufReader, prelude::*};

fn solve(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
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
    result
}

fn main() {
    println!("{}", solve("input"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day4_part2_test() {
        let solution = solve("test_input");
        assert_eq!(solution, 43);
    }
}
