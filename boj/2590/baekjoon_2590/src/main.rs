use std::io;

fn main() {
    let mut buf = String::new();
    let mut a: [i32; 6] = [0, 0, 0, 0, 0, 0];

    for i in 0..6 {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        a[i] = buf.trim().parse::<i32>().unwrap();
    }

    let mut n = a[5];

    let five = a[4];
    a[0] -= five * 11;

    if a[0] < 0 {
        a[0] = 0;
    }
    n += five;

    let four = a[3];
    let mut cap_2by2 = four * 5;

    if cap_2by2 < a[1] {
        a[1] -= cap_2by2;
    } else {
        cap_2by2 -= a[1];
        a[1] = 0;

        let cap_1by1 = cap_2by2 * 4;
        a[0] -= cap_1by1;

        if a[0] < 0 {
            a[0] = 0;
        }
    }
    n += four;

    let mut three = a[2] / 4;
    if a[2] % 4 != 0 {
        three += 1;
    }

    if a[2] % 4 == 1 {
        let mut cap_2by2 = 5;

        if a[1] > cap_2by2 {
            a[1] -= cap_2by2;
            let cap_1by1 = 9;
            a[0] -= cap_1by1;

            if a[0] < 0 {
                a[0] = 0;
            }
        } else {
            cap_2by2 -= a[1];
            a[1] = 0;
            let cap_1by1 = 7 + cap_2by2 * 4;
            a[0] -= cap_1by1;

            if a[0] < 0 {
                a[0] = 0;
            }
        }
    } else if a[2] % 4 == 2 {
        let mut cap_2by2 = 3;

        if a[1] > cap_2by2 {
            a[1] -= cap_2by2;
            let cap_1by1 = 6;
            a[0] -= cap_1by1;

            if a[0] < 0 {
                a[0] = 0;
            }
        } else {
            cap_2by2 -= a[1];
            a[1] = 0;
            let cap_1by1 = 6 + cap_2by2 * 4;
            a[0] -= cap_1by1;

            if a[0] < 0 {
                a[0] = 0;
            }
        }
    } else if a[2] % 4 == 3 {
        let mut cap_2by2 = 1;

        if a[1] > cap_2by2 {
            a[1] -= cap_2by2;
            let cap_1by1 = 5;
            a[0] -= cap_1by1;

            if a[0] < 0 {
                a[0] = 0;
            }
        } else {
            cap_2by2 -= a[1];
            a[1] = 0;
            let cap_1by1 = 5 + cap_2by2 * 4;
            a[0] -= cap_1by1;

            if a[0] < 0 {
                a[0] = 0;
            }
        }
    }
    n += three;

    let mut two = a[1] / 9;

    if a[1] % 9 != 0 {
        two += 1;
        let cap_1by1 = 36 - 4 * (a[1] % 9);
        a[0] -= cap_1by1;

        if a[0] < 0 {
            a[0] = 0;
        }
    }
    n += two;

    let mut one = a[0] / 36;

    if a[0] % 36 != 0 {
        one += 1;
    }
    n += one;

    println!("{}", n)
}
