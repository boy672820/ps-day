use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (a, b) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let gcd = euclidean(a, b);
    let lcm = (a / gcd) * (b / gcd) * gcd;
    println!("{}\n{}", gcd, lcm)
}

fn euclidean(mut a: i32, mut b: i32) -> i32 {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }
    b
}
