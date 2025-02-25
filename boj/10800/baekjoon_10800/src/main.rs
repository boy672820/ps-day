use io::Write;
use std::io;

const MAX: usize = 200_001;

struct Ball {
    color: i32,
    weight: i32,
    index: usize,
}

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    let t = buf.trim().parse::<usize>().unwrap();

    let mut temp: Vec<Ball> = Vec::new();

    for index in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (color, weight) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            (v[0], v[1])
        };

        temp.push(Ball {
            color,
            weight,
            index,
        })
    }

    temp.sort_by(|a, b| {
        if a.weight == b.weight {
            a.color.cmp(&b.color)
        } else {
            a.weight.cmp(&b.weight)
        }
    });

    let mut total_color = [0; MAX];
    let mut total_size = [0; MAX];
    let mut r = [0; MAX];

    let mut sum = 0;

    for i in 0..temp.len() {
        let color = temp[i].color;
        let weight = temp[i].weight;
        let index = temp[i].index;

        total_color[color as usize] += weight;
        total_size[weight as usize] += weight;
        sum += weight;

        r[index] = sum - total_color[color as usize] - total_size[weight as usize] + weight;

        if i != 0 && temp[i - 1].color == color && temp[i - 1].weight == weight {
            r[index] = r[temp[i - 1].index];
        }
    }

    for i in 0..t as usize {
        writeln!(out, "{}", r[i]).unwrap();
    }
}
