use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    for i in 1..n+1 {
        let mut m = i;
        let mut sum = 0;
        while m > 0 {
            let re = m % 10;
            sum += re;
            m /= 10;
        }
        if n == sum + i {
            println!("{}", i);
            break;
        }
        if i == n {
            println!("0")
        }
    }
}
