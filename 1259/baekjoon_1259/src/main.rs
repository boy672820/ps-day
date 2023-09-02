use std::io;

fn main() {
    let mut buf = String::new();
    let mut n = 1;

    while n != 0 {
        io::stdin().read_line(&mut buf).unwrap();
        n = buf.trim().parse::<u32>().unwrap();

        palindrome(n);

        buf.clear();
    }
}

fn palindrome(mut n: u32) {
    let mut i = 0;
    let mut a: [u32; 6] = [0; 6];
    while n != 0 {
        a[i] = n;
        n /= 10;
        i += 1;
    }

    println!("{:?}", a)
}