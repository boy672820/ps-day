use std::io::stdin;

fn main() {
    const RADIX: u32 = 10;

    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let v = buf.split_whitespace().collect::<Vec<_>>();

    let (a_str, b_str) = (v[0], v[1]);

    let a_rev = a_str.chars().rev().collect::<Vec<_>>().capacity(10001);
    let b_rev = b_str.chars().rev().collect::<Vec<_>>().capacity(10001);

    let mut res = Vec::new();
    let mut carry = 0;

    for i in 0..10001 {
        // let (a, b) = (
        //     a_rev[i].to_digit(RADIX).unwrap(),
        //     b_rev[i].to_digit(RADIX).unwrap(),
        // );
        let (a, b) = (a_rev[i], b_rev[i]);

        println!("{:?}, {:?}", a, b)

        // let r = a + b + carry;

        // if r > 9 {
        //     carry = 1;
        //     res.push((r % 10).to_string())
        // } else {
        //     carry = 0;
        //     res.push(r.to_string())
        // }
    }

    // if carry > 0 {
    //     res.push("1".to_string());
    // }

    // println!(
    //     "{}",
    //     res.iter()
    //         .rev()
    //         .fold(String::new(), |r, c| r + c.as_str() + "")
    // )
}
