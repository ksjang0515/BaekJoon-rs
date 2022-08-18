use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout());

    let mut buff = String::new();
    io::stdin().read_to_string(&mut buff).unwrap();
    //     let buff = r#"0 3 5 4 6 9 2 7 8
    // 7 8 2 1 0 5 6 0 9
    // 0 6 0 2 7 8 1 3 5
    // 3 2 1 0 4 6 8 9 7
    // 8 0 4 9 1 3 5 0 6
    // 5 9 6 8 2 0 4 1 3
    // 9 1 7 6 5 2 0 8 0
    // 6 0 3 7 0 1 9 5 2
    // 2 5 8 3 9 4 7 6 0"#
    //         .to_string();

    let mut board: [[u8; 9]; 9] = [[0; 9]; 9];

    for (i, line) in buff.lines().enumerate() {
        for (j, n) in line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .enumerate()
        {
            board[i][j] = n;
        }
    }

    if dfs(&mut board, 0) {
        for i in 0..9 {
            for j in 0..9 {
                write!(writer, "{} ", board[i][j]).unwrap();
            }
            write!(writer, "\n").unwrap();
        }
    } else {
        writeln!(writer, "Failed to find answer").unwrap();
    }
}

fn dfs(board: &mut [[u8; 9]; 9], x: usize) -> bool {
    if x == 81 {
        return true;
    }

    for i in x..81 {
        let (a, b) = (i / 9, i % 9);

        if board[a][b] == 0 {
            for n in 1..=9 as u8 {
                if check_safe(board, a, b, n) {
                    board[a][b] = n;

                    if dfs(board, i + 1) {
                        return true;
                    }
                }
            }
            board[a][b] = 0;
            return false;
        }
    }

    true
}

fn check_safe(board: &[[u8; 9]; 9], x: usize, y: usize, n: u8) -> bool {
    for i in 0..9 {
        if board[x][i] == n || board[i][y] == n {
            return false;
        }
    }

    let (a, b) = (x / 3 * 3, y / 3 * 3);
    for i in 0..9 {
        if board[a + i / 3][b + i % 3] == n {
            return false;
        }
    }

    true
}
