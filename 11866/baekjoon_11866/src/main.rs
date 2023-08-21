use std::collections::VecDeque;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, k) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let mut deq: VecDeque<usize> = VecDeque::new();
    for i in 1..n + 1 {
        deq.push_back(i);
    }

    let mut r = String::new();

    while deq.len() != 0 {
        for _ in 0..k-1 {
            deq.push_back(deq[0]);
            deq.remove(0);
        }
        r.push_str(&deq[0].to_string());
        r.push_str(", ");
        deq.remove(0);
    }

    println!("<{}>", &r[..r.len() - 2]);
}
