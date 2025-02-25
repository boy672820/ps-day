use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let count = buf.trim().parse::<u32>().unwrap();

    let mut v: Vec<i32> = Vec::new();

    for _ in 0..count {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<i32>().unwrap();

        v.push(n);
    }

    for i in 0..v.len() - 1 {
        for j in (i + 1)..v.len() {
            if v[i] > v[j] {
                let next = v[j];

                v[j] = v[i];
                v[i] = next;
            }
        }
    }

    for i in v.iter() {
        writeln!(out, "{}", i).unwrap();
    }
}
