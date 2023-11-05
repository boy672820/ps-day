use std::io;

const MAX: usize = 101;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<u32>().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let m = buf.trim().parse::<u32>().unwrap();

    let mut arr: [[bool; MAX]; MAX] = [[false; MAX]; MAX];

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (a, b) = {
            let v = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1])
        };

        // 그래프의 "관계"를 설정해 준다.
        arr[a][b] = true;
        arr[b][a] = true;
    }

    // 방문한 노드 ---> 인덱스로 구분함
    let mut checked: [bool; MAX] = [false; MAX];
    let mut count = 0;

    // 감염된 1번 컴퓨터(노드)와 관련된 노드를 탐색을 시작한다.
    dfs(1, &mut arr, &mut checked, &mut count);

    println!("{}", count)
}

/**
 * 깊이 우선 탐색
 * 
 * @param x 탐색할 노드의 인덱스
 * @param arr 그래프 자료구조를 담고 있음
 * @param checked 탐색한 노드의 인덱스를 담고 있음
 * @param count 방문한 횟수, 탐색한 노드의 경우 증가하지 않음
 */
fn dfs(x: usize, arr: &mut [[bool; MAX]; MAX], checked: &mut [bool; MAX], count: &mut u32) {
    checked[x] = true;

    // 지정된(x) 열부터 순차적으로 탐색을 시작한다.
    // 만약 ---> 1, 2, 3, ... 순차적으로 돌면서 3번에 관계가 형성 되었을 경우
    //     1. 방문을 체크하고 2차원 배열(arr)에 3번 열로 이동한다.
    //     2. 3번 열의 배열을 순회한다. ---> 1, 2, 3, ... 반복
    //     3. 형성된 관계가 없을 경우 첫번째 시작했던 행(x)의 순회를 종료한다.
    for i in 1..arr[x].len() {
        if arr[x][i] && !checked[i] {
            dfs(i, arr, checked, count);
            *count += 1;
        }
    }
}
