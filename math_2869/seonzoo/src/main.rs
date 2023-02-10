use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let vec = input
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let a = vec[0];
    let b = vec[1];
    let v = vec[2];

    let day = (v - b) / (a - b);

    println!("{}", day.ceil() as i32);
}
