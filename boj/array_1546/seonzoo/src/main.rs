use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let count = buf.trim().parse::<f64>().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();

    let read = buf
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let scores = read[0..count as usize].to_vec();
    let max = f64::from(*scores.iter().max().unwrap());

    let sum = scores.iter().map(|s| f64::from(*s) / max * 100.0).sum::<f64>();

    println!("{}", sum / count)
}
