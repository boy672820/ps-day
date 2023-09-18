use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let k = buf.trim().parse::<usize>().unwrap();
    let mut v: Vec<u32> = Vec::new();

    for _ in 0..k {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<u32>().unwrap();

        if n != 0 {
            v.push(n);
        } else {
            v.pop();
        }
    }

    let sum: u32 = v.iter().sum();
    println!("{}", sum)
}
