use std::collections::HashMap;
use std::io;
use io::Write;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    let mut map: HashMap<String, bool> = HashMap::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (name, status) = {
            let v = buf.split_whitespace().collect::<Vec<_>>();

            (v[0], v[1])
        };

        map.insert(
            name.to_string(),
            if status == "enter" { true } else { false },
        );
    }

    let mut r: Vec<&str> = Vec::new();

    for (k, v) in &map {
        if *v {
            r.push(k);
        }
    }

    r.sort_by(|a, b| b.cmp(a));

    for i in r.iter() {
        writeln!(out, "{}", i).unwrap();
    }
}
