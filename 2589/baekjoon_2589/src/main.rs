use std::collections::VecDeque;
use std::io;

const MAP_SIZE: usize = 50;
const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (h, w) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[0], v[1])
    };

    let mut m: [[i32; MAP_SIZE]; MAP_SIZE] = [[0; MAP_SIZE]; MAP_SIZE];

    for i in 0..h {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a = buf
            .split("")
            .map(|x| if x.contains("W") { 0 } else { 1 })
            .collect::<Vec<_>>();
        for j in 1..w + 1 {
            m[i][j - 1] = a[j];
        }
    }

    let mut visited: [[bool; MAP_SIZE]; MAP_SIZE] = [[false; MAP_SIZE]; MAP_SIZE];

    for i in 0..h {
        for j in 0..w {
            if m[i][j] == 1 {
                bfs(i, j, visited);
            }
        }
    }
}

struct Node {
    x: usize,
    y: usize,
    cost: usize,
}

fn bfs(x: usize, y: usize, mut visited: [[bool; MAP_SIZE]; MAP_SIZE]) {
    let mut deq: VecDeque<Node> = VecDeque::new();
    deq.push_front(Node { x, y, cost: 0 });
    visited[y][x] = true;

    let mut len = 0;

    while !deq.is_empty() {
        let cur = deq.pop_front();

        for i in 0..4 {
            let (nx, ny) = (cur.x as i32 + DX[i], cur.y as i32 + DY[i]);

            // if nx >= 0 && ny >= 0 && nx < h && ny < w
        }
    }
}
