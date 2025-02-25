use io::Write;
use std::io;

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<u32>().unwrap();

    for _ in 0..t {
        let v = [0; 2]
            .iter()
            .map(|_| {
                buf.clear();
                io::stdin().read_line(&mut buf).unwrap();

                buf.trim().parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>();

        let (k, n) = (v[0], v[1]);
        let mut arr: Vec<Vec<i32>> = Vec::new();

        for i in 0..k + 1 {
            let mut floors: Vec<i32> = Vec::new();

            for j in 0..n {
                let r = if i - 1 < 0 {
                    j + 1
                } else {
                    if j - 1 < 0 {
                        1
                    } else {
                        arr[(i - 1) as usize][j as usize] + floors[(j - 1) as usize]
                    }
                    // if j - 1 < 0 {
                    //     1
                    // } else {
                    //     arr[(i - 1) as usize][j as usize] - arr[i as usize][(j - 1) as usize]
                    // }
                };

                floors.push(r);
            }

            arr.push(floors);
        }

        writeln!(out, "{}", arr[k as usize][(n - 1) as usize]).unwrap()
        // println!("{:?}", arr[k as usize])
    }
}
