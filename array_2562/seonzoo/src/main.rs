use std::io::stdin;

fn main() {
    let mut buf = String::new();

    let mut max = 0;
    let mut index = 0;

    for i in 0..9 {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();

        let n: i32 = buf.trim().parse().unwrap();

        if n > max {
            max = n;
            index = i;
        }
    }

    println!("{}\n{}", max, index + 1);
}
