use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter, Write};

fn binary_search(vec: &Vec<isize>, num: isize) -> bool {
    let mut low = 0;
    let mut high = vec.len() - 1;
    loop {
        if high < low {
            return false;
        }
        let mid = (low + high) / 2;

        match num.cmp(&vec[mid]) {
            Ordering::Equal => return true,
            Ordering::Greater => low = mid + 1,
            Ordering::Less => {
                if mid == 0 {
                    return false;
                }
                high = mid - 1
            }
        }
    }
}

fn main() {
    let mut writer = BufWriter::new(stdout());

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s = "".to_string();
    stdin().read_line(&mut s).unwrap();

    let mut number_cards: Vec<isize> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<isize>>();

    number_cards.sort();

    stdin().read_line(&mut s).unwrap();
    s = "".to_string();
    stdin().read_line(&mut s).unwrap();
    for x in s.trim().split_whitespace() {
        let n = x.parse::<isize>().unwrap();
        if binary_search(&number_cards, n) {
            write!(writer, "1 ");
        } else {
            write!(writer, "0 ");
        }
    }
}
