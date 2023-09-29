use std::cmp;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (k, n) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1] as u32)
    };

    let mut v: Vec<u32> = Vec::new();
    let mut low = 1;
    let mut high = 0;
    let mut mid = 0;
    let mut r = 0;

    for _ in 0..k {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<u32>().unwrap();
        v.push(n);
        high = cmp::max(high, n);
    }

    while low <= high {
        mid = (low + high) / 2;
        let mut total = 0;

        for i in 0..k {
            total += v[i] / mid;
        }

        if total >= n {
            low = mid + 1;
            if r < mid {
                r = mid;
            }
        } else {
            high = mid - 1;
        }
    }

    println!("{}", r)
}
