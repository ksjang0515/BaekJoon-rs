use std::io::{self, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let stdin = io::stdin();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();

    let vec: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let result = vec[0] - vec[1];
    writeln!(writer, "{}", result).unwrap();
}
