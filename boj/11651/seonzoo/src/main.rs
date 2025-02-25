use io::Write;
use std::io;

fn main() {
    let mut buf = String::new();
    let mut out = io::BufWriter::new(io::stdout().lock());

    io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<u32>().unwrap();
    let mut v = Vec::new();

    for _ in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let r = buf
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        v.push(vec![r[1], r[0]]);
    }

    v.sort();

    for i in v.iter() {
        writeln!(out, "{} {}", i[1], i[0]).unwrap();
    }
}
