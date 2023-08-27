use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let _a = &buf
        .split_whitespace()
        .map(|x| x.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>()[..n];
    let mut a = _a.into_iter().collect::<Vec<_>>();
    a.sort();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let m = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let v = &buf
        .split_whitespace()
        .map(|x| x.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>()[..m];

    let mut r = String::new();
    for i in 0..v.len() {
        if binary_search(&a, v[i]) >= 0 {
            r.push_str("1\n");
        } else {
            r.push_str("0\n");
        }
    }

    println!("{}", &r[..r.len() - 1]);
}

fn binary_search(arr: &Vec<&isize>, target: isize) -> isize {
    let mut start = 0;
    let mut end = arr.len() as isize - 1;
    let mut r = -1;

    while start <= end {
        let mid = (start + end) / 2;
        if mid < 0 || (arr.len() as isize) <= mid {
            break;
        }

        let cur = *arr[mid as usize];

        if target < cur {
            end = mid - 1;
        } else if target > cur {
            start = mid + 1;
        } else {
            // return mid;
            r = mid;
            break;
        }
    }

    r
}
