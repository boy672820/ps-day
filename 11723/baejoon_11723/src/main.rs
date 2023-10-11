use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let m = buf.trim().parse::<usize>().unwrap();
    let mut a: [u32; 21] = [0; 21];
    let mut r = String::new();

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (action, x) = {
            let v = buf.split_whitespace().collect::<Vec<_>>();
            (
                v[0],
                if v[0].contains("all") || v[0].contains("empty") {
                    0
                } else {
                    v[1].trim().parse::<u32>().unwrap()
                },
            )
        };

        let (out, _a) = input(action, x, a);
        a = _a;
        if out != -1 {
            r.push_str(if out == 0 { "0\n" } else { "1\n" })
        }
    }

    println!("{}", &r[..r.len() - 1])
}

fn input(action: &str, x: u32, mut a: [u32; 21]) -> (i32, [u32; 21]) {
    let mut r = -1;

    match action {
        "add" => a[x] = 1,
        "remove" => a[x] = 0,
        "check" => r = if a[x] { 1 } else { 0 },
        "toggle" => a[x] = if a[x] { 0 } else { 1 },
        "all" => a = [1; 21],
        "empty" => a = [0; 21],
        _ => {}
    }

    (r, a)
}
