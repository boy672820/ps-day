use std::io::{self, BufRead};

fn main() {
    // let mut input = String::new();

    // std::io::stdin().read_line(&mut input).unwrap();

    // let mut count = 0;

    // while count < 2 {
    //     let v: i32 = input.trim().parse::<i32>().unwrap();

    //     println!("{}", count.to_string());

    //     if v
    //     count += 1;
    // }

    let mut lines = io::stdin().lock().lines();

    let mut i: Vec<i32> = Vec::new();
    let mut count: i32 = 1;

    while let Some(line) = lines.next() {
        let last_input: i32 = line.unwrap().parse::<i32>().unwrap();

        i.push(last_input);

        // stop reading
        if count >= 2 {
            break;
        }

        count += 1;
    }

    println!("{} {} {}", i[0], i[1], i[0] * i[1]);
}
