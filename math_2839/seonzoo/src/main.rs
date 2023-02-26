use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let input = buf.trim().parse::<i32>().unwrap();

    let mut n = input;
    let mut r = 0;

    while n > 0 {
        if n % 5 == 0 {
            r += n / 5;
            n = 0;

            break;
        }

        n -= 3;

        if n >= 0 {
            r += 1;
        }
    }

    println!("{}", if n == 0 { r } else { -1 })
}
