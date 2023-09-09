use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    // 216 = 198 + 1 + 9 + 8

    let mut m = n;

    println!("{}", m)
}
