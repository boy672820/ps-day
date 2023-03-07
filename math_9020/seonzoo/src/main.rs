use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let total_case = buf.trim().parse::<u32>().unwrap();

    for _ in 0..total_case {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<i32>().unwrap();

        let mut p1 = n / 2;

        loop {
            let p2 = n - p1;

            if is_prime(p2) && is_prime(p1) {
                writeln!(out, "{} {}", p1, p2).unwrap();
                break;
            }

            p1 -= 1;
        }
    }
}

fn is_prime(n: i32) -> bool {
    for i in 2..n / 2 + 1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
