use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let m: usize = 10_007;
    let n = buff.lines().next().unwrap().parse::<usize>().unwrap();

    let mut v: Vec<usize> = vec![1; 10];

    for _ in 2..=n {
        let mut new_v: Vec<usize> = vec![0; 10];

        new_v[0] = v[0];
        for i in 1..=9 {
            for j in 0..=i {
                new_v[i] += v[j];
            }
            new_v[i] = new_v[i] % m;
        }

        v = new_v;
    }

    let sum: usize = v.iter().sum::<usize>() % m;
    writeln!(writer, "{}", sum).unwrap();
}
