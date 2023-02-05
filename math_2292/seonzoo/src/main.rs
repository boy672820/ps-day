use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();
    let mut i = 1;
    let mut c = 0;

    if n == 1 {
        println!("1")
    } else {
        loop {
            if n <= i {
                break;
            }

            i += 6 * c;
            c += 1;
        }

        println!("{}", c)
    }
}
