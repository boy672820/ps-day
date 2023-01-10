use std::io;
use std::io::Write;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let t = buffer.trim().parse::<u32>().unwrap();

    for _ in 0..t {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();

        let n = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        writeln!(out, "{}", n[0] + n[1]);
    }
}
