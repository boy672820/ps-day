use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();
    let c = 0;

    if n == 1 {
        println!("{}", 0)
    } else {
        println!("{}", calc(n, c))
    }
}

fn calc(n: u32, mut c: u32) -> u32 {
    let r = if n % 3 == 0 {
        n / 3
    } else if n % 2 == 0 {
        n / 2
    } else {
        n - 1
    };

    c += 1;

    if r == 1 {
        c
    } else {
        calc(r, c)
    }
}
