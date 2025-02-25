use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let v = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let (n, mut k) = (v[0], v[1]);
    let mut r = 0;

    for i in 1..n + 1 {
        if k == 0 {
            break;
        }

        if n % i == 0 {
            k -= 1;
            r = i;
        }
    }

    println!("{}", if k != 0 { 0 } else { r })
}
