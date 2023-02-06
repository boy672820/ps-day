use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    let mut d = 0;
    let mut m = 0;
    let mut d_edge = 2;
    let mut m_edge = 1;

    let mut i = 1;
    let mut j = 1;

    for _ in 0..n {
        d += i;
        m += j;

        if m == m_edge {
            m_edge += 2;
            j = -1;
        } else if m < 1 {
            m = 1;
            j = 1;
        }

        if d == d_edge {
            d_edge += 2;
            i = -1;
        } else if d < 1 {
            d = 1;
            i = 1;
        }
    }

    println!("{}/{}", m, d)
}
