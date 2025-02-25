use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let mut n = buf.trim().parse::<i32>().unwrap();
    let mut div = 2;

    while n > 1 {
        if n % div == 0 {
            writeln!(out, "{}", div);
            n /= div;
        } else {
            div += 1;
        }
    }
}
