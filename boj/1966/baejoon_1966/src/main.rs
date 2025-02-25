use io::Write;
use std::collections::VecDeque;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<u32>().unwrap();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, m) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1])
        };

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let c = buf.split_whitespace().collect::<Vec<_>>();
        let mut deque: VecDeque<[u32; 2]> = VecDeque::new();
        for i in 0..n {
            deque.push_back([c[i as usize].trim().parse::<u32>().unwrap(), i])
        }

        let mut count: u32 = 0;

        while !deque.is_empty() {
            let front = deque.pop_front().unwrap();
            let mut is_max = true;

            for i in 0..deque.len() {
                if deque[i][0] > front[0] {
                    deque.push_back(front);
                    for _ in 0..i {
                        let f = deque.pop_front().unwrap();
                        deque.push_back(f);
                    }
                    is_max = false;
                    break;
                }
            }

            if !is_max {
                continue;
            }

            count += 1;
            if front[1] == m {
                break;
            }
        }

        writeln!(out, "{}", count).unwrap();
    }
}
