use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let n = buff.lines().next().unwrap().parse::<usize>().unwrap();

    let mut v: [usize; 2] = [0, 1];

    for _ in 2..=n {
        let new_v: [usize; 2] = [v[0] + v[1], v[0]];
        v = new_v;
    }

    let cnt: usize = v.iter().sum();
    writeln!(writer, "{}", cnt).unwrap();
}
