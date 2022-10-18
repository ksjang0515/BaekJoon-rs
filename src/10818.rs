use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let vec: Vec<isize> = buff
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    let maximum = vec.iter().max().unwrap();
    let minimum = vec.iter().min().unwrap();

    writeln!(writer, "{} {}", minimum, maximum);
}
