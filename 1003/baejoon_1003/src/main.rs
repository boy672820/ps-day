use io::Write;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<u32>().unwrap();

    let mut out = io::BufWriter::new(io::stdout().lock());

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let mut dp: [[usize; 2]; 41] = [[0; 2]; 41];
        dp[0][0] = 1;
        dp[1][1] = 1;

        let n = buf.trim().parse::<usize>().unwrap();
        let _r = fibonacci(n, &mut dp);

        writeln!(out, "{} {}", dp[n][0], dp[n][1]).unwrap();
    }
}

fn fibonacci(n: usize, dp: &mut [[usize; 2]; 41]) -> [usize; 2] {
    if n >= 2 && (dp[n][0] == 0 || dp[n][1] == 0) {
        let left = fibonacci(n - 1, dp);
        let right = fibonacci(n - 2, dp);

        dp[n][0] = left[0] + right[0];
        dp[n][1] = left[1] + right[1];
    }
    dp[n]
}
