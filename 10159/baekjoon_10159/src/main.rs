use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let m = buf.trim().parse::<u32>().unwrap();

    let mut arr: [[i32; 101]; 101] = [[0; 101]; 101];

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (a, b) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1])
        };

        arr[a][b] = 1;
        arr[b][a] = -1;
    }

    for k in 1..(n+1) {
        for i in 1..(n+1) {
            for j in 1..(n+1) {
                if arr[i][k] == arr[k][j] && arr[i][k] != 0 {
                    arr[i][j] = arr[i][k];
                }
            }
        }
    }

    for i in 1..(n+1) {
        let mut cnt = n - 1;
        for j in 1..(n+1) {
            if arr[i][j] != 0 {
                cnt -= 1;
            }
        }
        println!("{}", cnt)
    }
}
