use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut out = BufWriter::new(stdout().lock());
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse::<u32>().unwrap();

    let mut v: Vec<u32> = Vec::new();
    let mut s: Vec<String> = Vec::new();
    let mut keys: [usize; 201] = [0; 201];

    for i in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();

        let (age, name): (u32, String) = {
            let v = input.split_whitespace().collect::<Vec<_>>();

            (v[0].trim().parse::<u32>().unwrap(), v[1].to_string())
        };

        v.push(age);
        s.push(name);

        keys[age as usize] = i as usize;
    }

    v.sort_by(|a, b| b.cmp(a));

    for age in v.iter() {
        writeln!(out, "{} {}", age, s[keys[*age as usize]]).unwrap();
    }
}
