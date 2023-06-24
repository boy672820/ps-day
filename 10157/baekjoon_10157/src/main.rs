use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (c, r) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        (v[0], v[1])
    };

    let mut a = [[0; 10]; 10];

    for i in 0..(c + 1) {
        a[0][i] = -1;
        a[r + 1][i] = -1;
    }

    for i in 0..(r + 1) {
        a[i][0] = -1;
        a[i][c + 1] = -1;
    }

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let mut k = buf.trim().parse::<u32>().unwrap();

    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];
    let (mut x, mut y, mut dir, mut value) = (0, 0, 0, 1);

    loop {
        a[x][y] = value;

        if a[x + dx[dir]][y + dy[dir]] != 0 {}
    }
}
