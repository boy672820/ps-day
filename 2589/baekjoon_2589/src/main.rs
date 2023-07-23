use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (h, w) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let mut m: [[i32; 8]; 8] = [[0; 8]; 8];

    for i in 0..h {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a = buf
            .split("")
            .map(|x| if x.contains("W") { 0 } else { 1 })
            .collect::<Vec<_>>();
        for j in 1..w + 1 {
            m[i][j - 1] = a[j];
        }
    }

    println!("{:?}", m)
}
