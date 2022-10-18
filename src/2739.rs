use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let n = buff.lines().next().unwrap().parse::<usize>().unwrap();

    for i in 1..10 {
        writeln!(writer, "{} * {} = {}", n, i, n * i);
    }
}
