use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let (a, b) = (v[0], v[1]);

        if a == 0 && b == 0 {
            break;
        }

        if a / b == 0 {
            writeln!(out, "factor").unwrap()
        } else if a % b == 0 {
            writeln!(out, "multiple").unwrap()
        } else {
            writeln!(out, "neither").unwrap()
        }
    }
}
