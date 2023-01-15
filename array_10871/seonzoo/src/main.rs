use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let iter = buf.split_whitespace().collect::<Vec<&str>>();
    let n = iter[0].parse::<usize>().unwrap();
    let x = iter[1].parse::<i32>().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();

    let seq = buf
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut output = String::new();

    for i in 0..n {
        if seq[i] < x {
            output.push_str(&seq[i].to_string());
            output.push(' ');
        }
    }

    println!("{}", output)
}
