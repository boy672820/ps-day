use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();
    let mut total = 0;

    for i in 1..n {
        total += i;
    }

    println!("{}", total + n);
}
