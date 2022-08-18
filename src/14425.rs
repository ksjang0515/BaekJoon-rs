use std::collections::HashSet;
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let mut stdin = io::stdin();

    let mut buffer: String = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    //     let buffer = r#"4
    // 9 5 4 8"#
    //         .to_string();

    let mut set: HashSet<String> = HashSet::new();

    let mut iter = buffer.lines();
    let length = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    for _ in 0..length[0] {
        let value = iter.next().unwrap().to_string();
        set.insert(value);
    }

    let mut cnt = 0;
    for _ in 0..length[1] {
        let value = iter.next().unwrap().to_string();
        if set.contains(&value) {
            cnt += 1;
        }
    }

    writeln!(writer, "{}", cnt).unwrap();
}
