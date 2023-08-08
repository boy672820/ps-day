use std::io;

fn main() {
    let mut buf = String::new();
    let mut a: [usize; 6] = [0, 0, 0, 0, 0, 0];

    for i in 0..6 {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        a[i] = buf.trim().parse::<usize>().unwrap();
    }

    let mut n = a[5];

    while a[0] != 0 || a[1] != 0 || a[2] != 0 || a[3] != 0 || a[4] != 0 {
        while a[4] > 0 {
            let mut b = 36;
            a[4] -= 1;
            b -= 25;

            a[0] = if a[0] <= b { 0 } else { a[0] - b };
            n += 1;
        }
    }

    println!("{}", n)
}
