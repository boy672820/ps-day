use std::collections::HashMap;
use std::io;

fn main() {
    let v = input();
    let v2 = input();

    let map: HashMap<_, _> = v.iter().zip(v.iter()).collect();

    let mut r = String::new();

    for i in v2.iter() {
        if map.contains_key(i) {
            r.push_str("1 ")
        } else {
            r.push_str("0 ")
        }
    }

    println!("{}", &r[..&r.len() - 1])
}

fn input() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let v = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>()[..n]
        .to_vec();

    v
}
