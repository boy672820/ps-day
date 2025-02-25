use std::io;
use io::Write;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let total = buf.trim().parse::<usize>().unwrap();
    let mut v: Vec<i32> = Vec::new();

    for _ in 0..total {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<i32>().unwrap();

        v.push(n);
    }

    v.sort();

    for i in v.iter() {
        writeln!(out, "{}", i).unwrap();
    }
}
