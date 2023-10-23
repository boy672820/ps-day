use io::Write;
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

    let mut map: HashMap<String, String> = HashMap::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (host, pw) = {
            let v = buf.split_whitespace().collect::<Vec<_>>();
            (v[0], v[1])
        };

        map.insert(host.clone().to_string(), pw.clone().to_string());
    }

    let mut out = io::BufWriter::new(io::stdout().lock());

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let k = &buf[..buf.len() - 1];

        if map.contains_key(k) {
            writeln!(out, "{}", map.get(k).unwrap()).unwrap();
        }
    }
}
