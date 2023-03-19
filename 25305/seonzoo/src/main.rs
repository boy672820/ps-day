use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let (n, k) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        (v[0], v[1])
    };

    buf.clear();
    stdin().read_line(&mut buf).unwrap();

    let mut v = buf
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    for i in 1..v[0..n].len() {
        for j in 0..i + 1 {
            if v[i] > v[j] {
                let prev = v[i];
                v[i] = v[j];
                v[j] = prev;
            }
        }
    }

    println!("{:?}", v[(k - 1) as usize])
}
