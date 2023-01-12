use io::stdin;
// use io::stdout;
// use io::Write;
use std::io;

fn main() {
    // let stdout = stdout();
    // let mut out = io::BufWriter::new(stdout.lock());

    let mut buffer = String::new();

    loop {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

        let n: Vec<&str> = buffer.split_whitespace().collect();

        if n.len() == 0 {
            break;
        }

        let a: i32 = n[0].parse().unwrap();
        let b: i32 = n[1].parse().unwrap();

        // writeln!(out, "{}", a + b).unwrap();
        println!("{}", a + b);
    }
}
