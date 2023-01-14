use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let x = buffer.trim().parse::<i32>().unwrap();

    for _ in 0..1 {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

        let y = buffer.trim().parse::<i32>().unwrap();

        if x >= 0 && y >= 0 {
            println!("1");
        } else if x < 0 && y >= 0 {
            println!("2");
        } else if x < 0 && y < 0 {
            println!("3");
        } else {
            println!("4");
        }
    }
}
