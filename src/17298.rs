use std::io::{self, BufWriter, Read, Write};

struct Element {
    value: i32,
    index: usize,
}
fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let mut stdin = io::stdin();

    let mut buffer: String = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    //     let buffer = r#"4
    // 9 5 4 8"#
    //         .to_string();

    let mut stack: Vec<Element> = Vec::new();
    let mut nge: Vec<i32> = Vec::new();

    let iter = buffer
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    for (idx, value) in iter.enumerate() {
        nge.push(-1);

        while !stack.is_empty() {
            if stack.last().unwrap().value < value {
                nge[stack.pop().unwrap().index] = value;
            } else {
                break;
            }
        }
        stack.push(Element { value, index: idx });
    }

    for elem in nge {
        write!(writer, "{} ", elem).unwrap();
    }
}
