use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, k) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let c = factorial(n) / (factorial(k) * factorial(n - k));
    println!("{}", c)
}

fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}