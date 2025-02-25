use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let v = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let (a, b, c, d, e, f) = (v[0], v[1], v[2], v[3], v[4], v[5]);

    for x in -999..1000 {
        for y in -999..1000 {
            if a * x + b * y == c && d * x + e * y == f {
                println!("{} {}", x, y);
                break;
            }
        }
    }
}
