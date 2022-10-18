use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let line = buff;

    for (i, c) in line.chars().enumerate() {
        write!(writer, "{}", c);
        if (i + 1) % 10 == 0 {
            write!(writer, "\n");
        }
    }
}
