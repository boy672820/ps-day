use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let v: Vec<i32> = input
        .as_mut_str()
        .split_ascii_whitespace()
        .map(|x: &str| x.parse().unwrap())
        .collect();

    let a: i32 = v[0];
    let b: i32 = v[1];
    let c: i32 = v[2];

    let f = (a + b) % c;
    let s = ((a % c) + (b % c)) % c;
    let t = (a * b) % c;
    let fo = ((a % c) * (b % c)) % c;

    println!("{}\n{}\n{}\n{}", f, s, t, fo);
}
