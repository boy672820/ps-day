use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let mut v = [0; 26];

    for c in input.chars() {
        let code = c as i32;

        if code <= 10 {
            break;
        }

        if ((code - 65) as usize) > 25 {
            v[(code - 97) as usize] += 1;
        } else {
            v[(code - 65) as usize] += 1;
        }
    }

    let mut max_i = 0;
    let mut max_v = 0;
    let mut i = 0;

    for v in v.iter() {
        if v == &max_v {
            max_i = -1;
        } else if v > &max_v {
            max_v = *v;
            max_i = i;
        }

        i += 1;
    }

    let a = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    if max_i < 0 {
        println!("?")
    } else {
        println!("{}", a[max_i as usize]);
    }
}
