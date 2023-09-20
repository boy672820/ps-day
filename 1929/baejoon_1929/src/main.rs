use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (m, n) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let mut v: Vec<u32> = Vec::new();
    for x in m..n + 1 {
        if is_prime(x) {
            v.push(x);
        }
    }

    let mut r = String::new();
    for n in v.iter() {
        r.push_str(&n.to_string());
        r.push_str("\n");
    }
    println!("{}", if r.len() == 0 { &r } else { &r[..r.len() - 1] })
}

fn is_prime(x: u32) -> bool {
    if x == 1 {
        return false;
    }
    for i in 2..((x as f64).sqrt() as u32) + 1 {
        if x % i == 0 {
            return false;
        }
    }
    true
}
