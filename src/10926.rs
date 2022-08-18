use std::io::{self, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let stdin = io::stdin();

    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    let s = buffer.trim();

    writeln!(writer, "{}??!", s).unwrap();
}
