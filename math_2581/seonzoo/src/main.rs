use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let m = buf.trim().parse::<i32>().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i32>().unwrap();

    let mut p: Vec<i32> = Vec::new();

    for i in m..n + 1 {
        if is_prime(i) {
            p.push(i);
        }
    }

    if p.len() > 0 {
        println!("{}\n{}", p.iter().sum::<i32>(), p[0])
    } else {
        println!("{}", -1)
    }
}

fn is_prime(i: i32) -> bool {
    if i == 1 {
        return false;
    }

    for j in 2..(i / 2 + 1) {
        if i % j == 0 {
            return false;
        }
    }

    true
}
