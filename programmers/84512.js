console.log(solution('AAAAE')); // 6
console.log(solution('AAAE')); // 10
console.log(solution('AAAEA')); // 11
console.log(solution('AAAEU')); // 15
console.log(solution('AAAI')); // 16
console.log(solution('I')); // 1563
console.log(solution('EIO')); // 1189

function solution(word) {
  let result = {};
  let index = 0;
  // let arr = ["A", "E", "I", "O", "U"];
  let arr = ["A", "E", "I"];

  const dfs = (now, cnt) => {
    console.log(now);
    if (cnt > 5) return;

    result[now] = index;
    index += 1;

    for (let i = 0; i < 3; i += 1) {
      let next = now + arr[i]; // A -> AA -> AAA -> AAB -> AAC -> AB
      dfs(next, cnt + 1);
    }
  }

  dfs("", 0);
  return result[word];
}