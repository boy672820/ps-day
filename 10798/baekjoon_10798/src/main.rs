use std::io;

fn main() {
    let mut buf = String::new();
    let mut v: Vec<Vec<u8>> = Vec::new();

    let mut len = 0;

    for _ in 0..5 {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        v.push(buf.as_bytes().to_vec());

        len = if buf.len() > len { buf.len() } else { len };
    }

    let mut r = String::new();

    for i in 0..len {
        for j in 0..5 {
            if v[j].len() > i && v[j][i] != 10 {
                r.push(v[j][i] as char);
            }
        }
    }

    println!("{}", r)
}
