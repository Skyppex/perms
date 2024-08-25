use itertools::Itertools;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();
    let permutations = lines.iter().permutations(lines.len());
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for perm in permutations {
        match writeln!(handle, "{}", perm.iter().join(";")) {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}
