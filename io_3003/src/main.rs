use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let v: Vec<isize> = input
        .as_mut_str()
        .split_ascii_whitespace()
        .map(|x: &str| x.parse().unwrap())
        .collect();

    let pice_len = vec![1, 1, 2, 2, 2, 8];

    println!(
        "{} {} {} {} {} {}",
        pice_len[0] - v[0],
        pice_len[1] - v[1],
        pice_len[2] - v[2],
        pice_len[3] - v[3],
        pice_len[4] - v[4],
        pice_len[5] - v[5],
    )
}
