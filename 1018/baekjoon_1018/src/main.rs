use std::io;

// const MAX_SIZE: usize = 51;
const MAX_SIZE: usize = 5;

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
    let mut a: [[usize; MAX_SIZE]; MAX_SIZE] = [[0; MAX_SIZE]; MAX_SIZE];

    for m in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        for n in 0..n {
            a[m][n] = if buf.as_bytes()[n] as char == 'W' {
                0
            } else {
                1
            };
        }
    }

    println!("{:?}", a);
}
