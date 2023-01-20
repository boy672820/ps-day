use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut buf = String::new();

    let mut hash = HashMap::new();

    for _ in 0..10 {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<i32>().unwrap();
        let re = n % 42;

        hash.insert(re, hash.get(&re).unwrap_or(&0) + 1);
    }

    let mut count = 0;

    hash.iter().for_each(|(_, v)| {
        if *v > 0 {
            count += 1;
        }
    });

    println!("{}", count);
}
