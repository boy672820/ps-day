use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let l = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();

    let v = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut c = 0;

    for i in 0..l {
        let n = v[i];

        if n != 1 {
            c += 1;
        }

        for j in 2..(n / 2 + 1) {
            if n % j == 0 {
                c -= 1;
                break;
            }
        }
    }

    println!("{}", c)
}
