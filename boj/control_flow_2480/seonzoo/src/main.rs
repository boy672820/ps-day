use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();

    let n: Vec<u32> = buffer
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let one = n[0];
    let two = n[1];
    let three = n[2];

    if one == two && two == three && one == three {
        println!("{}", one * 1_000 + 10_000);
    } else if one == two || two == three || one == three {
        println!(
            "{}",
            if one == two {
                one
            } else if two == three {
                two
            } else {
                one
            } * 100
                + 1_000
        );
    } else {
        println!("{}", one.max(two).max(three) * 100);
    }
}
