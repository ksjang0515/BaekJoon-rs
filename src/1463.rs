use std::cmp::min;
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let n = buff.lines().next().unwrap().parse::<usize>().unwrap();
    let mut dp: Vec<usize> = vec![0; n + 1];

    for i in 2..n + 1 {
        let mut m = dp[i - 1] + 1;
        if i % 2 == 0 {
            m = min(dp[i / 2] + 1, m);
        }
        if i % 3 == 0 {
            m = min(dp[i / 3] + 1, m);
        }

        dp[i] = m;
    }

    writeln!(writer, "{}", dp[n]).unwrap();
}
