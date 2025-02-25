use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    let mut len = 0;

    for _ in 0..n {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();

        let mut v = [0; 123];
        let chars = buf.chars().collect::<Vec<_>>();

        len += 1;

        for i in 0..chars.len() {
            if chars.len() > (i + 1) && chars[i] == chars[i + 1] {
                continue;
            }

            if v[chars[i] as usize] > 0 {
                len -= 1;
                break;
            }

            v[chars[i] as usize] = 1;
        }
    }

    println!("{}", len)
}
