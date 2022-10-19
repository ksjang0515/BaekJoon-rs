use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let m: usize = 1_000_000_000;
    let n = buff.lines().next().unwrap().parse::<usize>().unwrap();

    if n == 1 {
        writeln!(writer, "9").unwrap();
        return;
    }

    let mut v: Vec<usize> = vec![1, 1, 2, 2, 2, 2, 2, 2, 2, 1];

    for _ in 3..=n {
        let mut new_v: Vec<usize> = vec![0; 10];

        new_v[0] = v[1];
        for i in 1..=8 {
            new_v[i] = v[i - 1] + v[i + 1] % m;
        }
        new_v[9] = v[8];

        v = new_v;
    }

    let sum: usize = v.iter().sum::<usize>() % m;
    writeln!(writer, "{}", sum).unwrap();
}
