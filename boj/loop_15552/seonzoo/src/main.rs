use io::Write;
use std::io;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let count = buffer.trim().parse::<u32>().unwrap();

    for _ in 0..count {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();

        let p = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();

        writeln!(out, "{}", p[0] + p[1]).unwrap();
    }
}
