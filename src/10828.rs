use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let stdin = io::stdin();
    let mut s: String = String::new();
    stdin.read_line(&mut s).unwrap();
    let number_of_commands = s.trim().parse::<u16>().unwrap();

    let mut stack: Vec<u32> = vec![];

    for _ in 0..number_of_commands {
        s.clear();
        stdin.read_line(&mut s).unwrap();
        let command: Vec<String> = s.trim().split_whitespace().map(|x| x.to_string()).collect();
        if command[0] == "push" {
            let num = command[1].parse::<u32>().unwrap();
            stack.push(num);
        } else if command[0] == "pop" {
            match stack.pop() {
                Some(num) => writeln!(writer, "{}", num),
                None => writeln!(writer, "-1"),
            };
        } else if command[0] == "size" {
            writeln!(writer, "{}", stack.len());
        } else if command[0] == "empty" {
            if stack.len() == 0 {
                writeln!(writer, "1");
            } else {
                writeln!(writer, "0");
            }
        } else if command[0] == "top" {
            if stack.len() == 0 {
                writeln!(writer, "-1");
            } else {
                let num = stack.last().unwrap();
                writeln! {writer, "{}", num};
            }
        } else {
            writeln!(writer, "Wrong Command").unwrap();
        }
    }
}
