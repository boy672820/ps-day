use std::io;

fn main() {
    let mut buf = String::new();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let a = buf.trim().parse::<i32>().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let b = buf.trim().parse::<i32>().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let c = buf.trim().parse::<i32>().unwrap();

    let mut mul = a * b * c;
    let mut arr: [u32; 10] = [0; 10];

    while mul != 0 {
        arr[(mul % 10) as usize] += 1;
        mul /= 10;
    }

    let mut r = String::new();

    for i in 0..arr.len() {
        r.push_str(&arr[i].to_string());
        r.push('\n');
    }

    println!("{}", &r[..r.len()-1]);
}
