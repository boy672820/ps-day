use std::cmp;
use std::io;

const MAX_SIZE: usize = 51;
const WHITE_BOARD: [[usize; 8]; 8] = [
    [0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 0],
];
const BLACK_BOARD: [[usize; 8]; 8] = [
    [1, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0, 1, 0, 1],
];

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, m) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    // 0 = White, 1 = Black
    let mut board: [[usize; MAX_SIZE]; MAX_SIZE] = [[0; MAX_SIZE]; MAX_SIZE];

    for i in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        for j in 0..m {
            board[i][j] = if buf.as_bytes()[j] as char == 'W' {
                0
            } else {
                1
            };
        }
    }

    let mut r = 37;
    for y in 0..n - 7 {
        for x in 0..m - 7 {
            let [w, b] = count_chess_board(board, x, y);
            let cmp = cmp::min(w, b);
            if cmp < r {
                r = cmp;
            }
        }
    }

    println!("{}", r)
}

fn count_chess_board(board: [[usize; MAX_SIZE]; MAX_SIZE], x: usize, y: usize) -> [usize; 2] {
    let mut white_count = 0;
    let mut black_count = 0;
    for i in 0..8 {
        for j in 0..8 {
            if WHITE_BOARD[i][j] != board[y + i][x + j] {
                white_count += 1;
            }
            if BLACK_BOARD[i][j] != board[y + i][x + j] {
                black_count += 1;
            }
        }
    }
    [white_count, black_count]
}
