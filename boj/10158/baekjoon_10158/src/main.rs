use std::io;

fn main() {
    let (w, h, mut p, mut q, t) = input();

    p += t;
    q += t;
    p %= 2 * w;
    q %= 2 * h;
    if p > w {
        p = 2 * w - p;
    }
    if q > h {
        q = 2 * h - q;
    }

    println!("{} {}", p, q)
}

fn input() -> (i32, i32, i32, i32, i32) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (w, h) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let (p, q) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<i32>().unwrap();

    (w, h, p, q, t)
}
