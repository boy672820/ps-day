/**
 *    0  1  2  3  4
 * 0 [1, 0, 1, 1, 1]
 * 1 [1, 0, 1, 0, 1]
 * 2 [1, 0, 1, 1, 1]
 * 3 [1, 1, 1, 0, 1]
 * 4 [0, 0, 0, 0, 1]
 * 
 * BFS 탐색
 */

console.log(solution([[1, 0, 1, 1, 1], [1, 0, 1, 0, 1], [1, 0, 1, 1, 1], [1, 1, 1, 0, 1], [0, 0, 0, 0, 1]]));
// console.log(solution([[1, 0, 1, 1, 1], [1, 0, 1, 0, 1], [1, 0, 1, 1, 1], [1, 1, 1, 0, 0], [0, 0, 0, 0, 1]]));

function solution(maps) {
  const dx = [1, 0, -1, 0]; // 동, 남, 서, 북
  const dy = [0, 1, 0, -1]; // 동쪽과 남쪽을 먼저 탐색하면서 내려가야함

  const n = maps.length;
  const m = maps[0].length;
  const visited = Array.from({ length: n }, () => Array(m).fill(false)); // 방문 여부

  function inRange(y, x) {
    return 0 <= y && y < n && 0 <= x && x < m; // 범위 체크
  }

  const queue = [[0, 0, 1]]
  let result = -1
  while (queue.length !== 0) {
    const [y, x, distance] = queue.shift()

    if (y === n - 1 && x === m - 1) {
      result = distance
      break
    }

    visited[y][x] = true
    for (let i = 0; i < 4; i++) {
      const newY = y + dy[i];
      const newX = x + dx[i];
      if (inRange(newY, newX) && maps[newY][newX] === 1 && !visited[newY][newX]) {
        queue.push([newY, newX, distance + 1])
      }
    }
  }
  return result
}