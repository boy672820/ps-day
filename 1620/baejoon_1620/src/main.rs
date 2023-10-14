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

    let mut hash: HashMap<String, usize> = HashMap::new();
    let mut v: Vec<String> = Vec::new();

    for i in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let slice = &buf[..buf.len() - 1];
        hash.insert(slice.clone().to_string(), i + 1);
        v.push(slice.clone().to_string());
    }

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let k = buf.trim().parse::<usize>();

        match k {
            Ok(n) => {
                writeln!(out, "{}", v[n - 1]).unwrap();
            }
            Err(_) => {
                let s = buf.trim();
                writeln!(out, "{}", hash.get(s).unwrap()).unwrap();
            }
        }
    }
}
