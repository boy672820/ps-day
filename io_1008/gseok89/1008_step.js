const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.on('line', function(line){
  const input = line.split(' ');

  const result = parseInt(input[0]) / parseInt(input[1]);
  console.log(result);

  rl.close();
}).on('close', function(){
  process.exit();
});