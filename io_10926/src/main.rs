use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: String = input
        .trim()
        .chars()
        .map(|c| if c == '?' { '!' } else { c })
        .collect();

    println!("{}??!", input)
}
