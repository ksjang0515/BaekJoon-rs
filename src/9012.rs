use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(stdout());

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let number_of_lines = s.trim().parse::<usize>().unwrap();

    'outer: for _ in 0..number_of_lines {
        let mut parenthesis = String::new();
        stdin().read_line(&mut parenthesis).unwrap();
        parenthesis = parenthesis.trim().to_string();

        let mut cnt = 0;
        for elem in parenthesis.chars() {
            if elem == '(' {
                cnt += 1;
            } else {
                cnt -= 1;
            }
            if cnt < 0 {
                writeln!(writer, "NO").unwrap();
                continue 'outer;
            }
        }
        if cnt == 0 {
            writeln!(writer, "YES").unwrap();
        } else {
            writeln!(writer, "NO").unwrap();
        }
    }
}
