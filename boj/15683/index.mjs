import readline from 'readline';
import { solution } from './solution.mjs';

let input = []

readline.createInterface({
  input: process.stdin,
  output: process.stdout,
}).on('line', function (line) {
  input.push(line);
}).on('close', function () {
  const firstLine = input[0];
  const otherLines = input.slice(1);
  solution(firstLine.split(' ').map(Number), otherLines.map((line) => line.split(' ')));
  process.exit();
})