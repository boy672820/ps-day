use io::Write;
use std::io;

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<u32>().unwrap();

    let mut out = io::BufWriter::new(io::stdout().lock());

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let case = buf.split_whitespace().collect::<Vec<_>>();

        let i = case[0].trim().parse::<u32>().unwrap();

        let mut result = String::new();

        for c in case[1].chars() {
            for _ in 0..i {
                result += &c.to_string();
            }
        }

        writeln!(out, "{}", result).unwrap();
    }
}
