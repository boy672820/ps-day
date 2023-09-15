use std::io;

fn main() {
    let mut buf = String::new();
    let mut r = String::new();

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        if buf.eq(".\n") {
            break;
        }

        let mut brackets: Vec<u32> = Vec::new();

        let chars = buf.chars().collect::<Vec<_>>();

        for i in 0..chars.len() {
            let c = chars[i];

            if c == '(' {
                brackets.push(1);
            }
            if c == '[' {
                brackets.push(0);
            }

            if c == ')' {
                if brackets.is_empty() || brackets.last().unwrap() != &1 {
                    brackets.push(1);
                    break;
                } else {
                    brackets.pop();
                }
            }

            if c == ']' {
                if brackets.is_empty() || brackets.last().unwrap() != &0 {
                    brackets.push(0);
                    break;
                } else {
                    brackets.pop();
                }
            }
        }

        if brackets.len() == 0 {
            r.push_str("yes\n");
        } else {
            r.push_str("no\n");
        }
    }

    if r.len() > 0 {
        println!("{}", &r[..r.len() - 1])
    }
}
