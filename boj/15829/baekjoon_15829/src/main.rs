use std::io;

const M: f64 = 1234567891.0;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let chars = buf.chars().collect::<Vec<_>>();

    let mut sum: f64 = 0.0;
    let mut pow: f64 = 1.0;

    for i in 0..n {
        let seq = ((chars[i] as u32) - 96) as f64;
        sum += seq * pow % M;
        pow = pow * 31.0 % M;
    }

    println!("{}", sum % M)
}
