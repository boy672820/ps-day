use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let m = buf.trim().parse::<usize>().unwrap();
    let mut bit = 0; // 공집합으로 초기화
    let mut r = String::new();

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (action, x) = {
            let v = buf.split_whitespace().collect::<Vec<_>>();
            (
                v[0],
                if v[0].contains("all") || v[0].contains("empty") {
                    0
                } else {
                    v[1].trim().parse::<u32>().unwrap()
                },
            )
        };

        match action {
            "all" => bit = (1 << 21) - 1, // 꽉 찬 집합
            "empty" => bit = 0,           // 공집합
            // 요소 추가, x만큼 왼쪽으로 이동 후 or 연산
            // 예를 들어 x=3 일 경우, 왼쪽으로 시프트(<<)하면 비트는 1000 즉, 10진수로 3값을 말하게 된다.
            // or 연산으로 값이 있든 없든 추가할 수 있다.
            "add" => bit |= 1 << x,
            // 요소 삭제
            // x=3 일 때, 비트는 1000이다. 이를 반전하면 0111
            // and 연산하여 3을 0으로 만든다.
            // 예를 들어, {3, 2} => 1100 일 때, 반전된 3(0111)을 and 연산했을 때
            "remove" => bit &= !(1 << x),
            // XOR 연산으로 toggle 기능을 쉽게 구현할 수 있음
            "toggle" => bit ^= 1 << x,
            // 대응하는 x의 비트가 1일 때, 1을 반환
            "check" => r.push_str(if (bit & (1 << x)) != 0 { "1\n" } else { "0\n" }),
            _ => {}
        }
    }

    println!("{}", &r[..r.len() - 1])
}
