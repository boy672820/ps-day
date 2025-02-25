use std::cmp;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    let mut dp: [usize; 1000001] = [0; 1000001];

    for i in 2..(n + 1) {
        dp[i] = dp[i - 1] + 1;

        if i % 2 == 0 {
            dp[i] = cmp::min(dp[i], dp[i / 2] + 1);
        }

        if i % 3 == 0 {
            dp[i] = cmp::min(dp[i], dp[i / 3] + 1);
        }
    }

    println!("{}", dp[n]);
}
