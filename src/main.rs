use itertools::Itertools;
use std::io::{self, BufRead, Write};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    let k = args
        .get(1)
        .unwrap_or(&lines.len().to_string())
        .parse::<usize>()
        .unwrap_or(lines.len());

    let permutations = lines.iter().permutations(k);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for perm in permutations {
        match writeln!(handle, "{}", perm.iter().join(";")) {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}
