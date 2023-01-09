const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.on('line', function(line){
  const input = line.toString();

  console.log(input+'??!');

  rl.close();
}).on('close', function(){
  process.exit();
});