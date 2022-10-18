use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut stdin = io::stdin();
    let mut buff = String::new();
    stdin.read_to_string(&mut buff).unwrap();

    let mut iter = buff
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let x = iter.next().unwrap();
    let y = iter.next().unwrap();

    let mut num = y;
    for i in 1..x {
        let d = match i {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => 28,
            _ => 0,
        };
        num += d;
    }

    let week = match num % 7 {
        1 => "MON",
        2 => "TUE",
        3 => "WED",
        4 => "THU",
        5 => "FRI",
        6 => "SAT",
        0 => "SUN",
        _ => "",
    };

    writeln!(writer, "{}", week);
}
