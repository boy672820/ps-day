use std::collections::HashMap;
use std::io;

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

    let mut s: HashMap<String, bool> = HashMap::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        s.insert(buf.clone(), true);
    }

    let mut r = 0;

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        if s.contains_key(&buf) {
            r += 1;
        }
    }

    println!("{}", r)
}
