use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();
    let v = buff
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    writeln!(writer, "{}", v[0] + v[1]);
}
