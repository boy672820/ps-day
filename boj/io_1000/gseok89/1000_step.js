var readline = require('readline');

var rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.on('line', function(line){
  var input = line.split(' ');

  var result = parseInt(input[0]) + parseInt(input[1]);
  console.log(result);

  rl.close();
}).on('close', function(){
  process.exit();
});