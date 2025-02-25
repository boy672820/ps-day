console.log(solution(["119", "97674223", "1195524421"]));
console.log(solution(["123", "456", "789"]));
console.log(solution(["12", "123", "1235", "567", "88"]));
console.log(solution(["1", "2", "3", "4", "5", "6"]));
console.log(solution(["1"]));
console.log(solution(["02", "0621234567", "06112345678", "01021004364", "1234556667"]));
console.log(solution(["1", "1"]));

function solution(phoneBook) {
  return !phoneBook.sort().some((_, i) => {
    if (i === phoneBook.length - 1) return false;

    return phoneBook[i + 1].startsWith(phoneBook[i]);
  })
}