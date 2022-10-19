use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let v: Vec<usize> = buff
        .lines()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let m = v.iter().max().unwrap();

    let mut dp: Vec<usize> = vec![0; m + 1];
    dp[1] = 1;
    dp[2] = 2;
    dp[3] = 4;

    for n in v {
        let r = func(&mut dp, n);
        writeln!(writer, "{}", r).unwrap();
    }
}

fn func(dp: &mut Vec<usize>, n: usize) -> usize {
    if dp[n] != 0 {
        return dp[n];
    }

    let mut sum = 0;
    sum += func(dp, n - 1);
    sum += func(dp, n - 2);
    sum += func(dp, n - 3);

    dp[n] = sum;
    return dp[n];
}
