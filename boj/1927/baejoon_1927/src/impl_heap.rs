use io::Write;
use std::io;

const MAX: usize = 10_001;

// 최소 힙
struct Heap {
    arr: [i32; MAX],
    size: usize,
}

impl Heap {
    fn swap(&mut self, a: usize, b: usize) {
        let a_value = self.arr[a];

        self.arr[a] = self.arr[b];
        self.arr[b] = a_value;
    }

    fn insert(&mut self, x: i32) {
        self.size += 1;
        self.arr[self.size] = x;

        let mut current_index = self.size as isize;
        let mut parent_index = current_index / 2;

        // 부모 노드가 없다는 뜻이므로, 첫번째 노드는 힙 재정렬을 하지 않는다.
        while parent_index > 0
            && current_index > 0
            && self.arr[parent_index as usize] > self.arr[current_index as usize]
        {
            self.swap(parent_index as usize, current_index as usize);

            current_index = parent_index;
            parent_index /= 2;
        }
    }

    fn pop(&mut self) -> i32 {
        // 아무 데이터가 없음 = 0
        if self.size == 0 {
            return 0;
        }

        let root = self.arr[1]; // 루트 노드의 값(힙에 저장된 최소 값)
        let last = self.arr[self.size]; // 마지막 노드의 값

        // 마지막 노드르 루트 노드로 이동하고 0으로 초기화한다.
        self.arr[self.size] = 0;
        self.arr[1] = last;
        self.size -= 1;

        let mut current_index = 1;

        // 마지막 노드(오른쪽 노드)까지 반복
        while current_index * 2 + 1 <= self.size {
            // 자식 노드 왼쪽과 오른쪽
            let (left_index, right_index) = (current_index * 2, current_index * 2 + 1);

            // 자식 노드보다 크지 않을 경우 정착
            if self.arr[current_index] < self.arr[left_index]
                && self.arr[current_index] < self.arr[right_index]
            {
                break;
            }

            // 자식 노드: 왼쪽 노드와 오른쪽 노드 비교하여 스왑
            if self.arr[left_index] < self.arr[right_index] {
                self.swap(current_index, left_index);
                current_index = left_index;
            } else {
                self.swap(current_index, right_index);
                current_index = right_index;
            }
        }

        return root;
    }
}

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<usize>().unwrap();
    let arr: [i32; MAX] = [0; MAX];
    let size = 0;

    let mut heap = Heap { arr, size };

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let x = buf.trim().parse::<i32>().unwrap();

        if x == 0 {
            let min = heap.pop();
            writeln!(out, "{}", min).unwrap();
        } else {
            heap.insert(x);
        }
    }
}
