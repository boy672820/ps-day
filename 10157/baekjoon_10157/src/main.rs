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

    let mut a: [[i32; 1002]; 1002] = [[0; 1002]; 1002];

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
    let mut k = buf.trim().parse::<i32>().unwrap();

    let dx: [i32; 4] = [-1, 0, 1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];
    let (mut x, mut y, mut dir, mut value): (i32, i32, i32, i32) = (0, 0, 0, 1);

    loop {
        if value == k {
            println!("{} {}", y, r)
        }

        if x >= 0 && y >= 0 {
            a[x as usize][y as usize] = value;
        }

        let column = x + dx[dir as usize];
        let row = y + dy[dir as usize];

        if column >= 0 && row >= 0 && a[column as usize][row as usize] != 0 {
            dir = (dir + 1) % 4;
        }

        x += dx[dir as usize];
        y += dy[dir as usize];

        value += 1;

        if value > (c * r) as i32 {
            break;
        }
    }
}
