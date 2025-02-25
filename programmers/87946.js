console.log(solution(80, [[80, 20], [50, 40], [30, 10]])); // 3
console.log(solution(0, [[1, 1]])); // 0
console.log(solution(1, [[1, 1]])); // 1
console.log(solution(1, [[1, 1], [1, 1], [1, 1], [1, 1], [1, 1], [1, 1], [1, 1], [1, 1]])); // 1
console.log(solution(10, [[1, 1], [1, 1], [1, 1], [1, 1], [10, 1], [1, 1], [1, 1], [1, 1]])); // 8
console.log(solution(10, [[3, 2], [5, 5], [1, 1], [3, 8], [4, 4]])); // 3

function solution(k, arr) {
  let result = 0;
  const visited = Array(arr.length).fill(false);

  dfs(k, 0);
  return result;

  function dfs(k, cnt) {
    result = Math.max(result, cnt);

    for (let i = 0; i < arr.length; i += 1) {
      if (k < arr[i][0] || visited[i]) continue;

      visited[i] = true;
      dfs(k - arr[i][1], cnt + 1);
      visited[i] = false;
    }
  }
}
