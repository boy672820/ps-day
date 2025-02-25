use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, mut k) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let mut coins: Vec<usize> = Vec::new();

    for _ in 1..n + 1 {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let coin = buf.trim().parse::<usize>().unwrap();
        coins.push(coin);
    }

    coins.reverse();

    let mut count = 0;

    for coin in coins.into_iter() {
        if k >= coin {
            count += k / coin;
            k %= coin;
        }
    }

    println!("{}", count)
}
