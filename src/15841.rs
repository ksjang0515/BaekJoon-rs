use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(stdout());

    let max_digit = 100000000;

    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();

        let n: i128 = s.trim().parse().unwrap();
        if n == -1 {
            break;
        }

        let (mut a, mut b): (Vec<u32>, Vec<u32>) = (vec![1], vec![1]);

        for k in 3..=n {
            let temp = b.clone();
            for i in 0..a.len() {
                let sum = b[i] + a[i];
                b[i] = sum % max_digit;

                let quotient = sum / max_digit;
                if quotient > 0 {
                    if i + 1 == b.len() {
                        b.push(quotient);
                    } else {
                        b[i + 1] += quotient;
                    }
                }
            }

            a = temp;
        }

        write!(writer, "Hour {}: ", n).unwrap();
        write!(writer, "{}", b[b.len() - 1]).unwrap();
        for elem in b.iter().rev().skip(1) {
            write!(writer, "{:08}", elem).unwrap();
        }
        write!(writer, " cow(s) affected\n").unwrap();
        write!(writer, "{}", b.len());
    }
}
