use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    loop {
        if buf == "" {
            break;
        }

        let n: i32 = buf.trim().parse().unwrap();
    }

    println!("{}", "Hello, world!")
}
