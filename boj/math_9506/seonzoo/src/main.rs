use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<i32>().unwrap();

        if n == -1 {
            break;
        }

        let mut sum = 0;
        let mut s = String::new();

        for i in 1..(n / 2 + 1) {
            if n % i == 0 {
                sum += i;
                s.push_str(&i.to_string());
                s.push_str(" + ");
            }
        }

        if sum == n {
            writeln!(out, "{} = {}", n, &s[0..s.len() - 3]).unwrap();
        } else {
            writeln!(out, "{} is NOT perfect.", n).unwrap();
        }
    }
}
