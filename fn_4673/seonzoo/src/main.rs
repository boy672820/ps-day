use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut d = [0; 20001];

    for i in 1..10001 {
        d[dn(i)] = 1;

        if d[i] != 0 {
            continue;
        }

        writeln!(out, "{}", i).unwrap();
    }
}

fn dn(_i: usize) -> usize {
    let mut i: usize = _i;
    let mut res: usize = i;

    if i >= 10000 {
        res += i / 10000;
        i %= 10000;
    }

    if i >= 1000 {
        res += i / 1000;
        i %= 1000;
    }

    if i >= 100 {
        res += i / 100;
        i %= 100;
    }

    if i >= 10 {
        res += i / 10;
        i %= 10;
    }

    res += i;

    res
}
