use std::cmp::{max, min};
use std::io::{self, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let num = buffer.trim().parse::<usize>().unwrap();
    let mut cnt: usize = 0;
    let mut queens: [usize; 14] = [0; 14];

    dfs(&mut queens, 0, num, &mut cnt);

    writeln!(writer, "{}", cnt).unwrap();
}

fn dfs(queens: &mut [usize], row: usize, num: usize, cnt: &mut usize) {
    if row == num {
        *cnt += 1;
        return;
    }

    let x = row; // 2 queens cannot be located at same row

    for y in 0..num {
        if !check_safe(x, y, queens, row) {
            continue;
        }

        queens[row] = y;
        dfs(queens, row + 1, num, cnt);
    }
}

fn check_safe(x: usize, y: usize, queens: &[usize], row: usize) -> bool {
    for i in 0..row {
        // check column
        if y == queens[i] {
            return false;
        }
        // check diagonal
        if x - i == max(queens[i], y) - min(queens[i], y) {
            return false;
        }
    }

    return true;
}
