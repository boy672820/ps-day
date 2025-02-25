use std::io;

fn main() {
    let mut buf = String::new();
    let mut r = String::new();

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (a, b, c) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1], v[2])
        };

        if a == 0 && b == 0 && c == 0 {
            break;
        }

        let (mut n1, mut n2, mut n3) = (0, 0, 0);

        if a < b {
            if b < c {
                n1 = a;
                n2 = b;
                n3 = c;
            } else {
                n1 = a;
                n2 = c;
                n3 = b;
            }
        } else {
            if a < c {
                n1 = a;
                n2 = b;
                n3 = c;
            } else {
                n1 = b;
                n2 = c;
                n3 = a;
            }
        }

        if n1 * n1 + n2 * n2 == n3 * n3 {
            r.push_str("right\n");
        } else {
            r.push_str("wrong\n");
        }
    }

    println!("{}", &r[..r.len() - 1])
}
