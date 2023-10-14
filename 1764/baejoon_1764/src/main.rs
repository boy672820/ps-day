use io::Write;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut out = io::BufWriter::new(io::stdout().lock());

    let (n, m) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let mut hash: HashMap<String, u32> = HashMap::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let slice = &buf[..buf.len() - 1];
        hash.insert(slice.clone().to_string(), 0);
    }

    let mut count = 0;
    let mut v: Vec<String> = Vec::new();

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let slice = &buf[..buf.len() - 1];
        let value = hash.get(slice);
        match value {
            Some(_) => {
                // writeln!(out, "{}", slice).unwrap();
                v.push(slice.clone().to_string());
                count += 1
            }
            None => {}
        }
    }

    v.sort();

    writeln!(out, "{}", count).unwrap();

    for s in v.iter() {
        writeln!(out, "{}", s).unwrap();
    }
}
