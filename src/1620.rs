use std::collections::HashMap;
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let mut stdin = io::stdin();

    let mut buffer: String = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    //     let buffer = r#"4
    // 9 5 4 8"#
    //         .to_string();

    let mut map: HashMap<&str, usize> = HashMap::new();
    let mut vec: Vec<&str> = Vec::new();

    let mut iter = buffer.lines();
    let length = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    for i in 0..length[0] {
        let value = iter.next().unwrap();
        map.insert(value, i);
        vec.push(value);
    }

    for _ in 0..length[1] {
        let value = iter.next().unwrap();
        let idx = value.parse::<usize>();
        if idx.is_ok() {
            writeln!(writer, "{}", vec[idx.unwrap() - 1]).unwrap();
            continue;
        }

        writeln!(writer, "{}", map.get(value).unwrap() + 1).unwrap();
    }
}
