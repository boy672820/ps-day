use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i32>().unwrap();

    let (mut m_5, mut m_25, mut m_125) = (0, 0, 0);
    m_5 = n / 5;
    m_25 = n / 25;
    m_125 = n / 125;

    println!("{}", m_5 + m_25 + m_125)
}
