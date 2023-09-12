use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut n = buf.trim().parse::<u32>().unwrap();
    let mut sixsixsix = 666;

    while n > 0 {
        let mut m = sixsixsix;
        let mut r = 0;

        while m > 0 {
            if r == 3 {
                break;
            }
            if m % 10 == 6 {
                r += 1;
            } else {
                r = 0;
            }
            m /= 10;
        }

        if r == 3 {
            n -= 1;
        }

        sixsixsix += 1;
    }

    println!("{}", sixsixsix - 1)
}
