use std::io;

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let y = input.trim().parse::<u32>().unwrap();

    println!("{}", (y - 543).to_string());
}
