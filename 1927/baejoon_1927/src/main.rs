use std::io;

// const MAX: usize = 10_000;
const MAX: usize = 10;

struct Heap {
    arr: [i32; MAX],
    size: usize,
}

impl Heap {
    fn insert(&mut self, x: i32) {
        self.arr[self.size] = x;
        self.size += 1;
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    let mut arr: [i32; MAX] = [0; MAX];
    let mut size = 0;

    let mut heap = Heap { arr, size };

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let x = buf.trim().parse::<i32>().unwrap();
        heap.insert(x);
    }

    println!("{:?}", heap.arr);
}
