use std::io::stdin;

fn main() {
    let value: u32 = read_int();

    if value % 4 == 0 && value % 100 != 0 || value % 400 == 0 {
        println!("1");
    } else {
        println!("0");
    }
}

fn read_int() -> u32 {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().parse::<u32>().unwrap()
}
