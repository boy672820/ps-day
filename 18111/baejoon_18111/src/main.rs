use std::cmp;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, m, b) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        (v[0] as usize, v[1] as usize, v[2])
    };

    let mut v: Vec<Vec<u32>> = Vec::new();
    let mut max = 0;

    for i in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let w = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut v2: Vec<u32> = Vec::new();

        for j in w.iter() {
            max = cmp::max(max, *j);
            v2.push(*j);
        }

        v.push(v2);
    }

    // -------------------------------------------------

    let (mut dur, mut h) = (0, max);
    let mut is_success = false;

    while !is_success {
        let mut inven = b;
        let mut cur_h = h;

        for i in 0..n {
            for j in 0..m {
                let mut cur = v[i][j];

                // 블록 놓기 작업 = 1초 소요
                while cur < cur_h && inven != 0 {
                    cur += 1;
                    inven -= 1;
                    dur += 1;
                }
                // 블록 빼기 작업 = 2초 소요
                while cur > cur_h {
                    cur -= 1;
                    inven += 1;
                    dur += 2;
                }

                if cur != cur_h {
                    is_success = false;
                    break;
                } else {
                    is_success = true;
                }
            }
        }

        h -= 1;
    }

    println!("{} {}", dur, h)
}
