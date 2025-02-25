const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.on('line', function(line){
  const input = line.split(' ').map(Number);

  const chess = [1,1,2,2,2,8];

  let result = input.map((i, index) => {
    return chess[index] - i;
  });

  console.log(...result);

  rl.close();
}).on('close', function(){
  process.exit();
});