console.log(solution(10, 2)); // [4, 3]
console.log(solution(8, 1)); // [3, 3]
console.log(solution(24, 24)); // [8, 6]

function solution(brown, yellow) {
  for (var i = 3; i <= (brown + yellow) / i; i++) {
    var x = Math.floor((brown + yellow) / i);

    if ((x - 2) * (i - 2) === yellow) {
      break;
    }
  }

  return [x, i];
}

/**
 * 
 * brown: 24, yellow: 24
 * 
 * 1. i = 3; i <= 24 / 3 = 8; i ++
 *   x = 48 / 3 = 16
 *   if 46 === 24; false
 * return [16, 3]
 * 2. i = 4; i <= 24 / 4 = 6; i ++
 *   x = 48 / 4 = 12
 *   if (48 - 2) * (4 - 2) = 92 === 24; false
 * return [12, 4]
 * 3. i = 5; i <= 24 / 5 = 4; i ++
 *  x = 48 / 5 = 9
 *  if (48 - 2) * (5 - 2) = 92 === 24; false
 * return [9, 5]
 * 4. i = 6; i <= 24 / 6 = 4; i ++
 *   x = 48 / 6 = 8
 *   if 24 === 24; true
 * return [8, 6]
 */
