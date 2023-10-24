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

        let mut a: [usize; 41] = [0; 41];

        let n = buf.trim().parse::<usize>().unwrap();

        let _r = fibonacci(n, &mut a);

        writeln!(out, "{} {}", a[0], a[1]).unwrap();
    }
}

fn fibonacci(n: usize, a: &mut [usize]) -> usize {
    if n < 2 {
        a[n] += 1;
        return a[n];
    }

    a[n] = fibonacci(n - 1, a) + fibonacci(n - 2, a);
    a[n]
}
