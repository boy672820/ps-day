use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut n = buf.trim().parse::<u32>().unwrap();
    let mut count = 0;

    while n != 1 {
        if n % 3 == 0 {
            n /= 3;
            count += 1;
        } else {
            if (n - 1) % 3 == 0 {
                if n % 2 == 0 {
                    n /= 2;
                    count += 1;
                } else {
                    n = (n - 1) / 3;
                    count += 2;
                }
            } else if n % 2 == 0 {
                n /= 2;
                count += 1;
            } else {
                n -= 1;
                count += 1;
            }
        }
    }

    println!("{}", count);
}
