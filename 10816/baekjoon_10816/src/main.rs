use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let a = &buf
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>()[..n];
    let mut v = a.into_iter().collect::<Vec<_>>();
    v.sort();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let m = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let cmp_v = &buf
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>()[..m];

    for i in 0..m {
        print!("{} ", cmp_v[i]);
    }
}

fn lower_boundary(k: i32, v: &Vec<&i32>) -> u32 {
    let (mut lo, mut hi) = (0, v.len() as i32);

    while lo < hi {
        let mid = (lo + hi) / 2;

        if k <= *v[mid as usize] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    lo
}
