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

    while deq.len() != 0 {
        for i in 0..deq.len() {
            if deq[i] == k {
                println!("{}", deq[i]);
                deq.remove(i);
            } else {
                deq.swap_remove_back(i);
            }
        }
    }
}
