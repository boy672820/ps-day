use io::Write;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut out = io::BufWriter::new(io::stdout().lock());
    let t = buf.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<usize>().unwrap();
        let mut map: HashMap<String, u32> = HashMap::new();

        for _ in 0..n {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();

            let (_, b) = {
                let v = buf.split_whitespace().collect::<Vec<&str>>();

                (v[0], v[1])
            };

            map.insert(
                b.clone().to_string(),
                if map.contains_key(b) {
                    map.get(b).unwrap() + 1
                } else {
                    1
                },
            );
        }

        let mut total = 1;

        for (_, v) in map.drain() {
            total *= v + 1;
        }

        total -= 1;

        writeln!(out, "{}", total).unwrap();
    }
}
