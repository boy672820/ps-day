use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();

    let split = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut h = split[0];
    let mut m = split[1];

    for _ in 0..1 {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

        let t = buffer.trim().parse::<i32>().unwrap();

        h += t / 60;
        m += t % 60;

        if m >= 60 {
            h += 1;
            m -= 60;
        }

        if h >= 24 {
            h -= 24;
        }

        println!("{} {}", h, m)
    }
}
