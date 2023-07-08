use std::io;

fn main() {
    let (w, h, p, q, t) = input();
    let (mut x, mut y) = (p, q);

    let (mut x_, mut y_) = (1, 1);

    for _ in 0..t {
        if x == w {
            x_ = -1;
            y_ = 1;
        }
        if y == h {
            x_ = -1;
            y_ = -1;
        }
        if x == 0 {
            x_ = 1;
            y_ = 1;
        }
        if y == 0 {
            x_ = -1;
            y_ = 1;
        }

        x += x_;
        y += y_;
    }

    println!("{} {}", x, y)
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
