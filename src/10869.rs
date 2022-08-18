use std::io::{self, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let stdin = io::stdin();

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();

    let vec: Vec<isize> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    writeln!(writer, "{}", vec[0] + vec[1]).unwrap();
    writeln!(writer, "{}", vec[0] - vec[1]).unwrap();
    writeln!(writer, "{}", vec[0] * vec[1]).unwrap();
    writeln!(writer, "{}", vec[0] / vec[1]).unwrap();
    writeln!(writer, "{}", vec[0] % vec[1]).unwrap();
}
