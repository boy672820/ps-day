use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n1, n2, n3, n4, n5) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1], v[2], v[3], v[4])
    };

    println!("{}", (n1*n1 + n2*n2 + n3*n3 + n4*n4 + n5*n5)%10)
}
