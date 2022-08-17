use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let mut stdin = io::stdin();

    let mut buffer: String = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let num = buffer.trim().parse::<u32>().unwrap();

    let mut stack: Vec<u32> = Vec::new();
    let mut actions: Vec<char> = Vec::new();

    buffer.clear();
    stdin.read_to_string(&mut buffer).unwrap();

    let mut sequence = buffer.lines().map(|x| x.parse::<u32>().unwrap());
    let mut n = sequence.next().unwrap();

    let mut stack_iter = 1..=num;

    loop {
        if !stack.is_empty() && *stack.last().unwrap() == n {
            stack.pop();
            actions.push('-');
            n = match sequence.next() {
                Some(number) => number,
                None => break,
            };
        } else {
            let i = match stack_iter.next() {
                Some(number) => number,
                None => break,
            };
            stack.push(i);
            actions.push('+');
        }
    }

    if stack.len() != 0 {
        writeln!(writer, "NO").unwrap();
    } else {
        for elem in actions {
            writeln!(writer, "{}", elem).unwrap();
        }
    }
}
