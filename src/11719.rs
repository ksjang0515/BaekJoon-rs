use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    writeln!(writer, "{}", buff);
}
