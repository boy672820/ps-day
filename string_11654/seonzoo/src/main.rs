use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    println!("{}", buf.chars().next().unwrap() as i32);
}
