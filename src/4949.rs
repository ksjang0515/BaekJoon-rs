use std::io::{self, BufWriter, Read, Write};

#[derive(PartialEq)]
enum Parenthesis {
    Square,
    Round,
}

fn main() {
    let mut writer = BufWriter::new(io::stdout());
    let mut stdin = io::stdin();

    let mut buffer: String = String::new();
    stdin.read_to_string(&mut buffer).unwrap();

    'outer: for s in buffer.lines() {
        if s == "." {
            break;
        }
        let mut stack: Vec<Parenthesis> = vec![];
        // writeln!(writer, "{}", s);

        for c in s.chars() {
            if c == '(' {
                stack.push(Parenthesis::Round);
            } else if c == ')' {
                if stack.is_empty() || *stack.last().unwrap() != Parenthesis::Round {
                    writeln!(writer, "no");
                    continue 'outer;
                }
                stack.pop();
            } else if c == '[' {
                stack.push(Parenthesis::Square);
            } else if c == ']' {
                if stack.is_empty() || *stack.last().unwrap() != Parenthesis::Square {
                    writeln!(writer, "no");
                    continue 'outer;
                }
                stack.pop();
            }
        }
        if stack.is_empty() {
            writeln!(writer, "yes");
        } else {
            writeln!(writer, "no");
        }
    }
}
