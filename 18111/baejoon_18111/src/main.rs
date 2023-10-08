use std::cmp;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, m, b) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        (v[0] as usize, v[1] as usize, v[2])
    };

    let mut v: Vec<Vec<isize>> = Vec::new();
    let mut max = 0;
    let mut min = 256;

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let w = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let mut v2: Vec<isize> = Vec::new();

        for j in w.iter() {
            max = cmp::max(max, *j);
            min = cmp::min(min, *j);
            v2.push(*j);
        }

        v.push(v2);
    }

    // -------------------------------------------------

    let mut dur_time = 0x7f7f7f7f;
    let mut most_height = 0;

    for h in min..max + 1 {
        let mut up_count = 0;
        let mut down_count = 0;
        let mut total_owned_block = b;

        for i in 0..n {
            for j in 0..m {
                let gap = h - v[i][j];

                if gap == 0 {
                    continue;
                } else if gap < 0 {
                    down_count += isize::abs(gap);
                } else {
                    up_count += isize::abs(gap);
                }
            }
        }

        total_owned_block += down_count;

        if up_count <= total_owned_block {
            let t = down_count * 2 + up_count;

            if dur_time > t {
                dur_time = t;
                most_height = h;
            } else if dur_time == t && most_height < h {
                most_height = h;
            }
        }
    }

    println!("{} {}", dur_time, most_height)
}
