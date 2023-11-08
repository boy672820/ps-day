use std::io;
use io::Write;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut out = io::BufWriter::new(io::stdout().lock());

    let t = buf.trim().parse::<usize>().unwrap();
    let mut memo: [u32; 12] = [0; 12];

    memo[1] = 1;
    memo[2] = 2;
    memo[3] = 4;

    for i in 4..11 {
        memo[i] = memo[i - 1] + memo[i - 2] + memo[i - 3];
    }

    for _ in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<usize>().unwrap();

        writeln!(out, "{}", memo[n]).unwrap();
    }
}
