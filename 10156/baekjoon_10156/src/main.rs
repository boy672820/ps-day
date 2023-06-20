use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (k, n, m) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        (v[0], v[1], v[2])
    };

    let r = k * n;

    println!("{}", if r > m { r - m } else { 0 })
}
