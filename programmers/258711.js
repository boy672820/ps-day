console.log(solution([[2, 3], [4, 3], [1, 1], [2, 1]]));
// console.log(solution([[4, 11], [1, 12], [8, 3], [12, 7], [4, 2], [7, 11], [4, 8], [9, 6], [10, 11], [6, 10], [3, 5], [11, 1], [5, 3], [11, 9], [3, 8]]));

function solution(edges) {
  const map = {}

  for (const [start, end] of edges) {
    map[start] = map[start] ?? [0, 0]
    map[end] = map[end] ?? [0, 0]
    map[start][0]++
    map[end][1]++
  }

  console.log(map);

  let addedNode = 0
  let donutCnt = 0
  let lineCnt = 0
  let eightCnt = 0
  for (const [start, [given, received]] of Object.entries(map)) {
    if (given > 1 && received === 0) {
      addedNode = start
    } else if (given === 0) {
      lineCnt++
    } else if (given > 1 && received > 1) {
      eightCnt++
    }

    console.log('start: %s | addedNode: %s | lineCnt: %s | eightCnt: %s', start, addedNode, lineCnt, eightCnt)
  }

  donutCnt = map[addedNode][0] - lineCnt - eightCnt

  return [Number(addedNode), donutCnt, lineCnt, eightCnt]
}