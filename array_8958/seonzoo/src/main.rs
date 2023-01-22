use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let total = buf.trim().parse::<u32>().unwrap();

    for _ in 0..total {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        {
            writeln!(out, "{}", quiz(&buf)).unwrap()
        }
    }
}

fn quiz(value: &str) -> u32 {
    let mut seq: u32 = 0;
    let mut result: u32 = 0;

    for c in value.chars() {
        let s = c.to_string();

        if s == "O" {
            seq += 1;
            result += seq;
        } else if s == "X" {
            seq = 0;
        }
    }

    result
}
