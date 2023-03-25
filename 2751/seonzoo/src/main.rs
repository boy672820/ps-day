use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let total = buf.trim().parse::<usize>().unwrap();
    let mut v: Vec<i32> = Vec::new();

    for i in 0..total {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<i32>().unwrap();

        v.push(n);
    }

    println!("{:?}", merge_sort(v));
}

fn merge_sort(v: Vec<i32>) -> Vec<i32> {
    if v.len() < 2 {
        return v;
    }
}
