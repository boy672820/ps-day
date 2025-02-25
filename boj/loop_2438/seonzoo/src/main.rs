use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();

    let n: u32 = buffer.trim().parse().unwrap();

    for i in 1..=n {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }
}
