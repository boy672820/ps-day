use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    let mut r = String::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let mut v: Vec<u32> = Vec::new();

        let mut is_break = false;

        let chars = buf.chars().collect::<Vec<_>>();
        for i in 0..chars.len() - 1 {
            let c = chars[i];
            if c == '(' {
                v.push(1);
            } else {
                if v.is_empty() {
                    r.push_str("NO\n");
                    is_break = true;
                    break;
                }
                v.pop();
            }
        }

        if !is_break {
            if v.is_empty() {
                r.push_str("YES\n");
            } else {
                r.push_str("NO\n");
            }
        }
    }

    println!("{}", &r[..r.len() - 1]);
}
