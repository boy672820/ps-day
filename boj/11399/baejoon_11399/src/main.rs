use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let mut v = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .take(n)
        .collect::<Vec<_>>();

    v.sort();

    let mut total = 0;
    let mut prev = 0;

    for i in v.iter() {
        let sum = i + prev;
        total += sum;
        prev = sum;
    }

    println!("{}", total)
}
