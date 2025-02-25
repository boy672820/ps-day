use io::Write;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut out = io::BufWriter::new(io::stdout().lock());

    let t = buf.trim().parse::<usize>().unwrap();
    let mut memo: [usize; 101] = [0; 101];

    memo[1] = 1;
    memo[2] = 1;
    memo[3] = 1;
    memo[4] = 2;
    memo[5] = 2;
    memo[6] = 3;
    memo[7] = 4;
    memo[8] = 5;
    memo[9] = 7;
    memo[10] = 9;

    for i in 10..101 {
        memo[i] = memo[i - 5] + memo[i - 1];
    }

    for _ in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<usize>().unwrap();

        writeln!(out, "{}", memo[n]).unwrap();
    }
}
