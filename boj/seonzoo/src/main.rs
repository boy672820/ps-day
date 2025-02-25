use std::io::stdin;

fn main() {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();

    let mut i = 0;
    let mut result = 0;

    for c in buf.chars() {
        i += 1;

        result += c.to_digit(10).unwrap();

         if i == n {
            break;
         }
    }

    println!("{}", result);
}
