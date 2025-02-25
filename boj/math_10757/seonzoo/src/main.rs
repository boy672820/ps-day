use std::io::stdin;

fn main() {
    const RADIX: u32 = 10;

    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let v = buf
        .split_whitespace()
        .map(|x| x.chars().rev().collect())
        .collect::<Vec<Vec<_>>>();

    let mut res = Vec::new();
    let mut carry = 0;

    let len = if v[0].len() < v[1].len() {
        v[1].len()
    } else {
        v[0].len()
    };

    for i in 0..len {
        let (a, b) = (
            if v[0].len() <= i {
                0
            } else {
                v[0][i].to_digit(RADIX).unwrap()
            },
            if v[1].len() <= i {
                0
            } else {
                v[1][i].to_digit(RADIX).unwrap()
            },
        );

        let r = a + b + carry;

        if r > 9 {
            carry = 1;
            res.push((r % 10).to_string())
        } else {
            carry = 0;
            res.push(r.to_string())
        }
    }

    if carry > 0 {
        res.push("1".to_string());
    }

    println!(
        "{}",
        res.iter().rev().fold(String::new(), |r, c| r + c.as_str() + "")
    )
}
