use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    for (i, line) in buff.lines().skip(1).enumerate() {
        let v = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        writeln!(writer, "Case #{}: {}", i + 1, v[0] + v[1]);
    }
}
