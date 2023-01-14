use std::io::stdin;

fn main() {
    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let value: u32 = input.trim().parse::<u32>().unwrap();

    if value >= 90 {
        println!("A")
    } else if value >= 80 {
        println!("B")
    } else if value >= 70 {
        println!("C")
    } else if value >= 60 {
        println!("D")
    } else {
        println!("F")
    }
}
