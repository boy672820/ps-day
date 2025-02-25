use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());

    let mut v = input_vector();

    v.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(&b)
        } else {
            a.len().cmp(&b.len())
        }
    });

    for (i, s) in v.iter().enumerate() {
        if i + 1 < v.len() {
            if v[i + 1] == v[i] {
                continue;
            }
        }

        writeln!(out, "{}", s).unwrap();
    }
}

fn input_vector() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse::<u32>().unwrap();
    let mut v: Vec<String> = Vec::new();

    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        input.pop();

        v.push(input.clone());
    }

    v
}
