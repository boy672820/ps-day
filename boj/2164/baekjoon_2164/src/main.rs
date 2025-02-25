use std::collections::VecDeque;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<u32>().unwrap();
    let mut deq: VecDeque<u32> = VecDeque::new();

    for i in 1..n + 1 {
        deq.push_back(i);
    }

    while deq.len() != 1 {
        deq.pop_front();
        deq.push_back(deq[0]);
        deq.remove(0);
    }

    println!("{}", deq[0])
}
