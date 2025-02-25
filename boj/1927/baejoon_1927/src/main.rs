use io::Write;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let x = buf.trim().parse::<i32>().unwrap();

        if x == 0 {
            let root = heap.pop();

            match root {
                Some(Reverse(x)) => writeln!(out, "{}", x).unwrap(),
                None => writeln!(out, "0").unwrap(),
            }
        } else {
            heap.push(Reverse(x));
        }
    }
}
