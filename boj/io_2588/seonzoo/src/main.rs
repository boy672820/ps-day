use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();

    let mut i: Vec<String> = Vec::new();
    let mut count: i32 = 1;

    while let Some(line) = lines.next() {
        let last_input: String = line.unwrap();

        i.push(last_input);

        if count >= 2 {
            break;
        }

        count += 1;
    }

    let left = i[0].parse::<i32>().unwrap();
    let right = i[1].parse::<i32>().unwrap();

    i[1].chars().rev().for_each(|c| {
        let right = c.to_string().parse::<i32>().unwrap();

        println!("{}", left * right);
    });

    println!("{}", left * right);
}
