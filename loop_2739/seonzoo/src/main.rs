use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();

    let input = buffer.trim().parse::<u32>().unwrap();

    for i in 1..10 {
        println!("{} * {} = {}", input, i, input * i)
    }
}
