use io::Write;
use std::io;

fn main() {
    let mut buf = String::new();

    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut students = [0; 30];

    for _ in 0..28 {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<usize>().unwrap();

        students[n - 1] = 1;
    }

    students.iter().enumerate().for_each(|(i, &x)| {
        if x == 0 {
            writeln!(out, "{}", i + 1).unwrap();
        }
    });
}
