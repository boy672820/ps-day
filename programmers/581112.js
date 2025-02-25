/**
 * 0 < n,m <= 1,000,000
 */
function resolve(a, b) {
  const n = gcf(a, b);
  const m = (a * b) / n;
  return [n, m];

  // 최대공약수
  function gcf(a, b) {
    const min = Math.min(a, b);
    let result = 1;
    for (let n = 2; n <= min; n += 1) {
      if (a % n === 0 && b % n === 0) {
        result = Math.max(result, n);
      }
    }
    return result;
  }
}

console.log(resolve(3, 12));
console.log(resolve(2, 5));
console.log(resolve(4, 6));
// (3, 12) = (3, 12)
// (2, 5) = (1, 10)
// (4, 6) = (2, 12)
