use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    let mut v: Vec<u32> = Vec::new();

    if n == 0 {
        println!("0");
        return;
    }

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<u32>().unwrap();
        v.push(n);
    }

    v.sort();

    let r = ((n as f32) * 0.15).round() as usize;
    let mut sum = 0;

    for i in r..v.len() - r {
        sum += v[i];
    }

    println!("{}", (sum as f32 / (v.len() - r * 2) as f32).round())
}
