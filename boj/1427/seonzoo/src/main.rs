use std::io;
use io::Write;

const RADIX: u32 = 10;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.pop();

    let mut v = input
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap())
        .collect::<Vec<_>>();

    v.sort_by(|a, b| b.cmp(a));

    let mut out = io::BufWriter::new(io::stdout().lock());

    for i in v.iter() {
        write!(out, "{}", i).unwrap();
    }
}
