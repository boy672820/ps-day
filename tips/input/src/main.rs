use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_ascii_whitespace();
    let number: i32 = tokens.next().unwrap().parse().unwrap();

    println!("{}", number)
}
