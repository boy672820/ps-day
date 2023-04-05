use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let total = buf.trim().parse::<u32>().unwrap();

    let mut counting = [0; 10001];

    for _ in 0..total {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<u32>().unwrap();
        counting[n as usize] += 1;
    }

    for i in 0..10001 {
        for _ in 0..counting[i] {
            writeln!(out, "{}", i).unwrap();
        }
    }
}
