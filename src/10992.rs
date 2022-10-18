use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let n = buff.lines().next().unwrap().parse::<usize>().unwrap();

    writeln!(writer, "{}*", " ".repeat(n - 1));
    for i in 2..n {
        let line = format!("{}*{}*", " ".repeat(n - i), " ".repeat(2 * i - 3));

        writeln!(writer, "{}", line);
    }
    if n != 1 {
        writeln!(writer, "{}", "*".repeat(2 * n - 1));
    }
}
