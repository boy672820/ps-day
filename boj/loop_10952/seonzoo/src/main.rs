use io::stdin;
use io::stdout;
use io::Write;
use std::io;

fn main() {
    let mut buffer = String::new();

    let stdout = stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

        let n = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        if n[0] == 0 && n[1] == 0 {
            break;
        }

        writeln!(out, "{}", n[0] + n[1]).unwrap();
    }
}
