const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.on('line', function(line){
  const input = line.split(' ');

  const a = parseInt(input[0]);
  const b = parseInt(input[1]);
  const c = parseInt(input[2]);


  console.log((a + b) % c);
  console.log(((a % c) + (b % c)) % c);
  console.log((a * b % c));
  console.log(((a % c) * (b % c)) % c);

  rl.close();
}).on('close', function(){
  process.exit();
});