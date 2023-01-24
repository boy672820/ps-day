use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut count = 0;

    for i in 1..n + 1 {
        if i < 100 {
            count += 1;
        } else if i >= 1000 {
            break;
        } else {
            let mut t = i;
            let mut v = Vec::new();

            while t > 0 {
                v.push(t % 10);
                t /= 10;
            }

            if v[0] - v[1] == v[1] - v[2] {
                count += 1;
            }
        }
    }

    println!("{}", count)
}
