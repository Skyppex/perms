use itertools::Itertools;
use std::io::{self, BufRead, Write};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if let Some(a) = args.get(1) {
        if a == "-h" || a == "--help" {
            eprintln!("Usage: {} [k]", args[0]);
            std::process::exit(1);
        }
    }

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();

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
