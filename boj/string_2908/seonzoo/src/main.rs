use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let v = input
        .split_whitespace()
        .map(|x| x.chars().rev().collect::<String>().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", if v[0] < v[1] { v[1] } else { v[0] })
}
