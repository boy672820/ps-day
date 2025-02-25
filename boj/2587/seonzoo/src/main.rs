use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let mut v: Vec<i32> = Vec::new();
    let mut sum = 0;

    for _ in 0..5 {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<i32>().unwrap();
        sum += n;

        v.push(n);
    }

    for i in 0..4 {
        for j in (i + 1)..5 {
            if v[i] > v[j] {
                let next = v[j];

                v[j] = v[i];
                v[i] = next;
            }
        }
    }

    let avg = sum / 5;
    let median = v[2];

    println!("{}\n{}", avg, median)
}
