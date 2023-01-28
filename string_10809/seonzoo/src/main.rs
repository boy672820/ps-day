use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let mut a = [-1; 123];

    let mut i = 0;

    for c in input.chars() {
        if a[c as usize] == -1 {
            a[c as usize] = i;
        }

        i += 1;
    }

    println!(
        "{}",
        a[97..123]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
