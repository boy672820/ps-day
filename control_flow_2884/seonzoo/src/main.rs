use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let n: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let total = n[0] * 60 + n[1] - 45;
    let h = total / 60;
    let m = total % 60;

    if m < 0 {
        println!("{} {}", 23, m + 60);
    } else {
        println!("{} {}", h, m);
    }
}
