console.log(solution([["yellow_hat", "headgear"], ["blue_sunglasses", "eyewear"], ["green_turban", "headgear"]])); // 5
console.log(solution([["crow_mask", "face"], ["blue_sunglasses", "face"], ["smoky_makeup", "face"]])); // 3

function solution(clothes) {
  let key = new Map()
  // 모든 의상에 갯수를 구한다.
  // 예를 들어, 예제 1의 map은 다음과 같다.
  // { [headgear] => 2, [eyewear] => 1 }
  for (let i = 0; i < clothes.length; i++) {
    if (key.has(clothes[i][1])) {
      key.set(clothes[i][1], key.get(clothes[i][1]) + 1)
    } else {
      key.set(clothes[i][1], 1)
    }
  }

  let answer = 1
  for (let a of key.values()) {
    // 의상의 종류마다 가지고 있는 갯수를 구했으니,
    // 이를 이용하여 조합 수를 구할 수 있다.
    // 예를 들어, a 의상 갯수가 2개이고 b 의상 갯수가 3개이면 2*3=6이다.
    answer *= (a + 1)
    // +1 개를 하는 이유는 "아무것도 착용하지 않는다"는 조건이다.
    // 예를 들어, a의상은 착용했지만 b의상은 착용하지 않을 수 있다.
  }
  
  
  // -1은 '모든 의상을 착용하지 않았을 떄'의 조건을 빼는 것이다.
  return answer - 1
}

/*

문제 분석: 옷의 조합의 수 구하기
접근 방법: 해시를 이용한 순열 만들기
시간복잡도: O(30!)

[[1, a], [2, b], [3, c], [4, a]]

[1]
[1,2]    [1,3]
[1,2,3]  [1,3,2]

[2]
[2,1]    [2,3]             [2,4]
[2,1,3]  [2,3,4]  [2,3,1]  [2,4,1] [2,4,3]

a=2, b=2

*/