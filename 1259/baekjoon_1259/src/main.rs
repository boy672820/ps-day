use std::io;

fn main() {
    let mut buf = String::new();

    let mut r = String::new();

    loop {
        io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i32>().unwrap();

        if n == 0 {
            break;
        }

        if &buf.as_str()[0..1] == "0" {
            r.push_str("no\n");
            buf.clear();
            continue;
        }

        let chars = n.to_string().chars().collect::<Vec<_>>();
        let len = chars.len();
        let mut result = true;
        for i in 0..(len / 2) {
            if chars[i] == chars[len - i - 1] {
                result = true;
            } else {
                result = false;
                break;
            }
        }
        r.push_str(if result { "yes\n" } else { "no\n" });

        buf.clear();
    }

    println!("{}", &r[..r.len()-1]);
}
