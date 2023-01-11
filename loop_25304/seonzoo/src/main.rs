use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();

    let total_amount = buffer.trim().parse::<i32>().unwrap();

    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();

    let count = buffer.trim().parse::<i32>().unwrap();
    let mut calc_amount = 0;

    for _ in 0..count {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();

        let p = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        calc_amount += p[0] * p[1];
    }

    println!(
        "{}",
        if total_amount == calc_amount {
            "Yes"
        } else {
            "No"
        }
    );
}
