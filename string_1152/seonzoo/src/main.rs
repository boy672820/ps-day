use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let v = input.split_whitespace().collect::<Vec<_>>();

    println!("{}", v.len())
}
