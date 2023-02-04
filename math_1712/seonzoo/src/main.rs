use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let v = input
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let fixed_cost = v[0];
    let var_cost = v[1];
    let price = v[2];

    if var_cost > price {
        println!("-1")
    } else if var_cost == price {
        println!("-1")
    } else {
        let x = fixed_cost / (price - var_cost) + 1;

        println!("{}", x)
    }
}
