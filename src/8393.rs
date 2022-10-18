use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let n = buff.lines().next().unwrap().parse::<usize>().unwrap();

    // let s = (1..n + 1).sum::<usize>();
    let s = (n + 1) * n / 2;

    writeln!(writer, "{}", s);
}
