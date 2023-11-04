use std::cmp;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    let mut v: Vec<usize> = Vec::new();

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n = buf.trim().parse::<usize>().unwrap();

        v.push(n);
    }

    let mut memo: Vec<usize> = Vec::new();

    // 점화식(Bottom-Up) 초기 값 셋팅
    memo.push(v[0]);

    if n == 1 {
        println!("{}", memo[0]);
        return;
    }

    memo.push(cmp::max(v[0] + v[1], v[1])); // |시작점|o|o| VS |시작점|x|o|

    if n == 2 {
        println!("{}", memo[1]);
        return;
    }
    
    memo.push(cmp::max(v[0] + v[2], v[1] + v[2])); // |시작점|o|x|o| VS |시작점|x|o|o|

    for i in 3..n {
        // 마지막 칸은 무조건 밟고 가야 됨 --> ...|o|x|마지막칸| VS ...|o|x|o|마지막칸|
        memo.push(cmp::max(memo[i - 2] + v[i], memo[i - 3] + v[i - 1] + v[i]));
    }

    println!("{}", memo[n - 1])
}
