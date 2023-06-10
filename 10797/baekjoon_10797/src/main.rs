use std::io;

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let d = buf.trim().parse::<usize>().unwrap() % 10;

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let mut r = [0; 10];

    for i in buf.split_whitespace().into_iter() {
        r[i.trim().parse::<usize>().unwrap()] += 1;
    }

    println!("{}", r[d])
}
