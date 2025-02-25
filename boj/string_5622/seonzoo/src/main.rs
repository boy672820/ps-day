use std::io::stdin;

fn main() {
    let mut v = [0; 91];
    let mut cur = 65;
    let mut take = 3;

    for i in 65..91 {
        if 81 == i {
            cur = i;
        }

        if (i - cur) == 3 {
            take += 1;
            cur = i;
        }

        v[i] = take;
    }

    v[90] = 10;

    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    println!("{}", input.chars().map(|c| v[c as usize]).sum::<i32>());
}
