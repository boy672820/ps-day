const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

let input = [];

rl.on('line', function(line){
  input.push(line);
}).on('close', function(){

  let num1 = parseInt(input[0]);
  let num2 = parseInt(input[1]);

  const oneNum = num2 % 10;
  const tenNum = Math.floor((num2%100)/10);
  const hundredNum = Math.floor(num2/100);


  console.log(num1*oneNum);
  console.log(num1*tenNum);
  console.log(num1*hundredNum);
  console.log(num1*num2);

  process.exit();
});