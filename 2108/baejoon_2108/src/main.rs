use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    let mut v: Vec<i32> = Vec::new();
    let mut a: [u32; 8001] = [0; 8001];
    let mut count = 0.0;
    let mut sum = 0.0;
    let mut max = -4000;
    let mut min = 4000;

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim().parse::<i32>().unwrap();

        v.push(n);
        a[(4000 + n) as usize] += 1;
        sum += n as f64;
        count += 1.0;
        max = if max < n { n } else { max };
        min = if min > n { n } else { min };
    }

    v.sort();

    let avg: i32 = f64::round(sum / count) as i32;
    let middle = v[v.len() / 2];
    let range = (min - max) * -1;

    let mut modes: Vec<i32> = Vec::new();
    let mut max_count = 1;

    for i in 0..a.len() {
        if max_count < a[i] {
            max_count = a[i];
            modes = vec![(i as i32) - 4000];
        } else if max_count == a[i] {
            modes.push((i as i32) - 4000);
        } else {
        }
    }

    modes.sort();

    let mode = if modes.len() <= 1 { modes[0] } else { modes[1] };

    println!("{}\n{}\n{}\n{}", avg, middle, mode, range);
}
