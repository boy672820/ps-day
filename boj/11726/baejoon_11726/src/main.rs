use std::io;

const MODULAR: usize = 10_007;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    let mut memo: [usize; 1001] = [0; 1001];
    memo[1] = 1;
    memo[2] = 2;

    for i in 3..1001 {
        memo[i] = (memo[i - 2] + memo[i - 1]) % MODULAR;
    }

    println!("{}", memo[n]);
}
