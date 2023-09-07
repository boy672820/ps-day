use std::io;
use std::cmp;

fn main() {
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
    let v = &buf
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..n];

    let mut max = 0;
    for i in 0..v.len() {
        for j in i..v.len() {
            if v[i] == v[j] {
                continue;
            }
            for k in j..v.len() {
                if v[j] == v[k] {
                    continue;
                }
                if v[i] == v[k] {
                    continue;
                }
                let total = v[i] + v[j] + v[k];
                if total == m {
                    max = total;
                    break;
                } else if total <= m {
                    max = cmp::max(max, total);
                }
            }
        }
    }

    println!("{}", max)
}
