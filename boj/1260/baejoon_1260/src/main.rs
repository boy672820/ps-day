use std::collections::VecDeque;
use std::io::{self, Write};

type AdjacencyList = Vec<Vec<usize>>;

struct Graph {
    adjacency_list: AdjacencyList,
}

impl Graph {
    fn dfs(&self, start_node: usize, unvisited: &mut Vec<bool>, visited: &mut Vec<usize>) {
        unvisited[start_node] = false;

        visited.push(start_node);

        for &neighbor in &self.adjacency_list[start_node] {
            if unvisited[neighbor] {
                self.dfs(neighbor, unvisited, visited)
            }
        }
    }

    fn bfs(&self, start_node: usize, visited: &mut Vec<usize>) {
        let mut unvisited = vec![true; self.adjacency_list.len()];
        let mut q: VecDeque<usize> = VecDeque::new();

        unvisited[start_node] = false;

        q.push_back(start_node);

        while let Some(current_node) = q.pop_front() {
            visited.push(current_node);

            for &neighbor in &self.adjacency_list[current_node] {
                if unvisited[neighbor] {
                    unvisited[neighbor] = false;

                    q.push_back(neighbor)
                }
            }
        }
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (m, v) = {
        let v = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (v[1], v[2])
    };

    let mut adjacency_list: AdjacencyList = vec![vec![]; 1001];

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

        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }

    // 정렬을 통해 낮은 숫자부터 조회하게 함
    for i in 0..adjacency_list.len() {
        adjacency_list[i].sort();
    }

    let graph = Graph { adjacency_list };

    let mut unvisited = vec![true; graph.adjacency_list.len()];
    let mut dfs_visited: Vec<usize> = vec![];
    let mut bfs_visited: Vec<usize> = vec![];

    graph.dfs(v, &mut unvisited, &mut dfs_visited);
    graph.bfs(v, &mut bfs_visited);

    writeln!(
        &io::stdout(),
        "{}",
        dfs_visited
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
    .unwrap();

    writeln!(
        &io::stdout(),
        "{}",
        bfs_visited
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
    .unwrap();
}
