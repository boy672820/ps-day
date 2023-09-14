use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    let mut a: [[u32; 2]; 200] = [[0, 0]; 200];

    for i in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        a[i] = [v[0], v[1]];
    }

    for i in 0..n {
        let mut rate = 1;
        for j in 0..n {
            let ([target_weight, target_tall], [cur_weight, cur_tall]) = (a[i], a[j]);

            if target_weight < cur_weight && target_tall < cur_tall {
                rate += 1;
            }
        }
        println!("{}", rate)
    }
}
