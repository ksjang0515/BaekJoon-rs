use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let n = buff.lines().next().unwrap().parse::<usize>().unwrap();
    let mut fibo: Vec<usize> = vec![0; n + 1];
    fibo[0] = 1;
    fibo[1] = 1;

    for i in 2..=n {
        fibo[i] = (fibo[i - 1] + fibo[i - 2]) % 10_007;
    }

    writeln!(writer, "{}", fibo[n]).unwrap();
}
