use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let v = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (mut asc, mut desc) = (0, 0);
    for i in 0..4 {
        if v[i] == (i+1) && v[i] + v[7-i] == 9 {
            asc += 1;
        } else if v[7-i] == (i+1) && v[i] + v[7-i] == 9 {
            desc += 1;
        }
    }

    if asc == 4 {
        println!("ascending");
    } else if desc == 4 {
        println!("descending");
    } else {
        println!("mixed");
    }
}
