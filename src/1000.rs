use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let num: i32 = s
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .sum::<i32>();

    println!("{}", num);
}
