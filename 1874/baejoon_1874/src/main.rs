use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<u32>().unwrap();
    let mut v: Vec<u32> = Vec::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        v.push(buf.trim().parse::<u32>().unwrap());
    }

    let mut seq: Vec<u32> = Vec::new();
    let mut pop_index = 0;

    let mut r = String::new();

    for i in 1..n + 1 {
        seq.push(i);
        r.push_str("+\n");

        while seq.len() != 0 && seq[seq.len() - 1] == v[pop_index] {
            seq.pop();
            pop_index += 1;
            r.push_str("-\n");
        }
    }

    if seq.len() == 0 {
        println!("{}", &r[..r.len() - 1])
    } else {
        println!("NO")
    }
}
