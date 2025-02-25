use std::collections::HashMap;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let v = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>()[0..t]
        .to_vec();

    let mut sorted = v.to_vec();

    sorted.sort();
    sorted.dedup();

    let mut map: HashMap<i32, usize> = HashMap::new();

    for (v, k) in sorted.iter().enumerate() {
        if map.contains_key(k) {
            continue;
        }

        map.insert(*k, v);
    }

    let mut r = String::new();

    for i in v.iter() {
        let append = map.get(i).unwrap().to_string() + " ";
        r.push_str(&append)
    }

    println!("{}", r[0..r.len() - 1].to_string())
}
