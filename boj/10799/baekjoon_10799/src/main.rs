use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut s = Vec::new();
    let mut r = 0;

    let c: Vec<_> = buf.chars().collect();

    for i in 0..c.len() - 1 {
        if c[i] == '(' {
            s.push(c[i]);
        } else {
            s.pop();

            if c[i - 1] == '(' {
                r += s.len();
            } else {
                r += 1;
            }
        }
    }

    println!("{}", r)
}
