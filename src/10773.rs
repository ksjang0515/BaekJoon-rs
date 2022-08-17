use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let mut stdin = io::stdin();

    let mut buffer: String = String::new();
    stdin.read_to_string(&mut buffer).unwrap();

    let mut stack: Vec<u32> = vec![];

    for s in buffer.lines().skip(1) {
        let money: u32 = s.trim().parse().unwrap();
        if money == 0 {
            stack.pop();
        } else {
            stack.push(money);
        }
    }

    let sum: u32 = stack.into_iter().sum();
    writeln!(writer, "{}", sum).unwrap();
}
