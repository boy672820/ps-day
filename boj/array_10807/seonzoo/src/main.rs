use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();

    let len = buffer.trim().parse::<usize>().unwrap();

    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();

    let v = buffer
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();

    let target = buffer.trim().parse::<i32>().unwrap();

    let mut count = 0;

    for i in 0..len {
        if v[i] == target {
            count += 1;
        }
    }

    println!("{}", count)
}
