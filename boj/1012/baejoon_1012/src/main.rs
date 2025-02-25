use io::Write;
use std::collections::VecDeque;
use std::io;

type Matrix = [[bool; MAX]; MAX];
type Coord = (usize, usize);

const MAX: usize = 50;
const DIRECTIONS: [(isize, isize); 4] = [
    // (x, y)
    (0, -1), // 상
    (-1, 0), // 좌
    (1, 0),  // 우
    (0, 1),  // 하
];

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let t = buf.trim().parse::<u32>().unwrap();

    for _ in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (m, n, k) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1], v[2])
        };

        let mut matrix: Matrix = [[false; MAX]; MAX];

        for _ in 0..k {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();

            let (x, y) = {
                let v = buf
                    .split_whitespace()
                    .map(|x| x.trim().parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (v[0], v[1])
            };

            matrix[y][x] = true;
        }

        // 배추흰지렁이 찾기

        let mut unvisited: Matrix = [[true; MAX]; MAX];
        let mut count = 0;

        for y in 0..n {
            for x in 0..m {
                // 확인되지 않은 영역 & 배추가 있음 ===> BFS로 인접한 배추 영역을 찾음
                if unvisited[y][x] && matrix[y][x] {
                    let mut q: VecDeque<Coord> = VecDeque::new();

                    bfs(x, y, &matrix, &mut unvisited, &mut q);

                    unvisited[y][x] = false; // 탐색이 완료되었음
                    count += 1; // 배추흰지렁이를 배치할 카운트 +
                }
            }
        }

        writeln!(out, "{}", count).unwrap();
    }
}

/**
 * 너비 우선 탐색
 */
fn bfs(x: usize, y: usize, matrix: &Matrix, unvisited: &mut Matrix, q: &mut VecDeque<Coord>) {
    q.pop_front();

    let max = MAX as isize;

    // 상하좌우에 배추가 있는지 확인
    for (dx, dy) in DIRECTIONS {
        let target_x = x as isize + dx;
        let target_y = y as isize + dy;

        // 유효한 좌표인지 확인
        if target_x >= 0 && target_y >= 0 && target_x < max && target_y < max {
            let target_x = target_x as usize;
            let target_y = target_y as usize;

            // 해당 좌표에 배추가 있다면
            if unvisited[target_y][target_x] && matrix[target_y][target_x] {
                // 탐색 완료 처리
                unvisited[target_y][target_x] = false;
                // 그리고 인접한 배추(노드)를 큐에 저장 ===> 배추흰지렁이가 이동할 수 있기 때문
                q.push_front((target_x, target_y));
            }
        }
    }

    for i in 0..q.len() {
        if q.is_empty() {
            break;
        }

        let (new_x, new_y) = q[i];

        bfs(new_x, new_y, matrix, unvisited, q);
    }
}
