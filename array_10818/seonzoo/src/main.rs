use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();

    let seq = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!(
        "{} {}",
        seq[0..n].iter().min().unwrap(),
        seq[0..n].iter().max().unwrap()
    );
}
