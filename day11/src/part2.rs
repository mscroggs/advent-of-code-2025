use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn solve(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut machines = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(": ");
        let machine = String::from(parts.next().unwrap());
        let outputs = parts
            .next()
            .unwrap()
            .split(" ")
            .map(String::from)
            .collect::<Vec<_>>();
        machines.insert(machine, outputs);
    }

    let svr_dac = count_routes(&machines, "svr", "dac", vec!["dac", "fft"]);
    let svr_fft = count_routes(&machines, "svr", "fft", vec!["dac", "fft"]);
    let fft_out = count_routes(&machines, "fft", "out", vec![]);
    let dac_out = count_routes(&machines, "dac", "out", vec![]);
    let fft_dac = count_routes(&machines, "fft", "dac", vec!["dac"]);
    let dac_fft = count_routes(&machines, "dac", "fft", vec!["fft"]);

    svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
}

fn count_routes(
    machines: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    not_included: Vec<&str>,
) -> usize {
    let mut post = vec![];
    for ni in not_included {
        if machines.contains_key(ni) {
            post.push(String::from(ni));
            let mut new_post = vec![String::from(ni)];
            while !new_post.is_empty() {
                let mut new_new_post = vec![];
                for p in &new_post {
                    if let Some(next) = machines.get(p) {
                        for n in next {
                            if !post.contains(&n) {
                                post.push(String::from(n));
                                new_new_post.push(String::from(n));
                            }
                        }
                    }
                }
                new_post = new_new_post;
            }
        }
    }

    let mut paths = HashMap::from([(String::from(start), 1)]);
    let mut valid_paths = 0;

    loop {
        let mut new_paths = HashMap::new();

        for (p, n) in &paths {
            for next in &machines[p] {
                if next == end {
                    valid_paths += n;
                } else if post.contains(&next) {
                    continue;
                } else if machines.contains_key(next) {
                    if let Some(i) = new_paths.get_mut(next) {
                        *i += *n;
                    } else {
                        new_paths.insert(String::from(next), *n);
                    }
                }
            }
        }

        if new_paths.len() == 0 {
            break;
        }
        paths = new_paths;
    }

    valid_paths
}

fn main() {
    println!("{}", solve("input"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day11_part2_test() {
        let solution = solve("test_input_part2");
        assert_eq!(solution, 2);
    }
}
