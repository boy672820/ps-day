use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let value: Vec<isize> = input
        .as_mut_str()
        .split_ascii_whitespace()
        .map(|x: &str| x.parse().unwrap())
        .collect();

    if value[0] > value[1] {
        println!(">")
    } else if value[0] < value[1] {
        println!("<")
    } else {
        println!("==")
    }
}
