use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<u32>().unwrap();

    let mut temp: Vec<[u32; 3]> = Vec::new();

    for i in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (c, s) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            (v[0], v[1])
        };

        temp.push([c, s, i])
    }

    temp.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut r = [0; 200_001];

    for i in 0..temp.len() {
        let mut p = 0;

        for j in (i + 1)..temp.len() {
            if temp[i][0] != temp[j][0] {
                p += temp[j][1];
            }
        }

        r[temp[i][2] as usize] = p;
    }

    for i in 0..(t as usize) {
        writeln!(out, "{}", r[i]).unwrap();
    }
}
