export function main([n, m], map) {
  console.log(map)
}

const readline = require('readline').createInterface({
  input: process.stdin,
  output: process.stdout,
})

let input = []

readline.on('line', function (line) {
  input.push(line);
}).on('close', function () {
  const firstLine = input[0];
  const otherLines = input.slice(1);
  main(firstLine.split(' ').map(Number), otherLines.map((line) => line.split(' ')));
  process.exit();
})