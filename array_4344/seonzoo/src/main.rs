use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let size = buf.trim().parse::<u32>().unwrap();

    for _ in 0..size {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let total = n[0];
        let list = &n[1..(total + 1) as usize];
        let average = list.iter().sum::<u32>() / total;

        let mut count = 0;

        for i in list.iter() {
            if i > &average {
                count += 1;
            }
        }

        let result = count as f32 / total as f32 * 100.0;

        writeln!(out, "{:.3}%", result).unwrap()
    }
}
