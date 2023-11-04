use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<u32>().unwrap();
    let mut v: Vec<u32> = Vec::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<u32>().unwrap();

        v.push(n);
    }

    let mut t: Vec<u32> = Vec::new();

    // 첫 번째와 두 번째는 필수이기 때문에 추가 됨
    t.push(v[0]);
    t.push(*v.last().unwrap());

    // 마지막 요소는 필수이기 때문에, 순회할 때 제외 됨
    for i in 1..(v.len() - 1) {
    }

    println!("{:?}", t)
}
