use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<u32>().unwrap();

    for _ in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let h = v[0];
        let n = v[2];
        // let w = v[1];

        let calc = n % h;

        let y = if calc == 0 { h } else { n % h } * 100;
        let x = if calc == 0 { n / h } else { n / h + 1 };

        writeln!(out, "{}", y + x).unwrap();
    }
}
