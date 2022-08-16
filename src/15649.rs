use std::io::{stdin, stdout, BufWriter, Write};

fn func(n: usize, m: usize, v: &mut Vec<usize>, writer: &mut impl Write) {
    if v.len() == m {
        for elem in v {
            write!(writer, "{} ", elem).unwrap();
        }
        write!(writer, "\n").unwrap();
        return;
    }

    for i in 1..=n {
        if v.contains(&i) {
            continue;
        }

        v.push(i);
        func(n, m, v, writer);
        v.pop();
    }
}

fn main() {
    let mut writer = BufWriter::new(stdout());

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let numbers = s
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut v: Vec<usize> = vec![];
    func(numbers[0], numbers[1], &mut v, &mut writer);
}
