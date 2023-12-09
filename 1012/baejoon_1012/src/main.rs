use std::io;

// const MAX: usize = 20;
const MAX: usize = 50;
const up_down_left_right: [[isize; 2]; 4] = [[0, -1], [0, 1], [-1, 0], [0, 1]];

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<u32>().unwrap();

    for _ in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (m, n, k) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1], v[2])
        };

        let mut arr: [[bool; MAX]; MAX] = [[false; MAX]; MAX];

        for _ in 0..k {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();

            let (x, y) = {
                let v = buf
                    .split_whitespace()
                    .map(|x| x.trim().parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (v[0], v[1])
            };

            arr[y][x] = true;
        }

        // 배추흰지렁이 찾기

    }
}
