use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, m) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let nums = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>()[..n]
        .to_vec();

    let mut sums: [u32; 100_001] = [0; 100_001];
    let mut prev = nums[0];
    sums[1] = prev;

    for i in 1..nums.len() {
        sums[i + 1] = prev + nums[i];
        prev = sums[i + 1];
    }

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (i, j) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1])
        };

        let total = sums[j] - sums[i - 1];

        writeln!(out, "{}", total).unwrap();
    }
}
