use std::io::{self, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();

    let num = buff.trim().parse::<usize>().unwrap();
    let mut vec: Vec<Vec<char>> = vec![vec!['*'; num]; num];

    func(&mut vec, 0, 0, num);

    for v in vec {
        for c in v {
            write!(writer, "{}", c).unwrap();
        }
        write!(writer, "\n").unwrap();
    }
}

fn func(vec: &mut [Vec<char>], x: usize, y: usize, len: usize) {
    if len < 3 {
        return;
    }

    let third = len / 3;
    for i in 0..3 {
        for j in 0..3 {
            let a = x + third * i;
            let b = y + third * j;

            if i == 1 && j == 1 {
                for p in 0..third {
                    for q in 0..third {
                        vec[a + p][b + q] = ' ';
                    }
                }
                continue;
            }

            func(vec, a, b, third);
        }
    }
}
