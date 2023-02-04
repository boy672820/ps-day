use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let v: Vec<char> = input.chars().collect();
    let mut len = input.len();

    for i in 0..v.len() {
        let c = v[i];

        if c == 'c' || c == 's' || c == 'z' {
            if v[i + 1] == '=' {
                len -= 1;
            }
        }

        if c == 'c' || c == 'd' {
            if v[i + 1] == '-' {
                len -= 1;
            }
        }

        if c == 'l' || c == 'n' {
            if v[i + 1] == 'j' {
                len -= 1;
            }
        }

        if c == 'd' {
            if v[i + 1] == 'z' && v[i + 2] == '=' {
                len -= 1;
            }
        }
    }

    len -= 1;

    println!("{}", len)
}
