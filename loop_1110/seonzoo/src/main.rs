use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<i32>().unwrap();

    let mut next = n;
    let mut count = 1;

    loop {
        let f = next / 10;
        let l = next % 10;

        let sum = f + l;

        next = l * 10 + sum % 10;

        if next == n {
            break;
        }

        count += 1;
    }

    println!("{}", count)
}
