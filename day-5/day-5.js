const fs = require('fs');

const list = fs.readFileSync('./day-5/input.txt').toString().split('\n');

const go = list => {
  var moves = 0;
  var position = 0;
  for (; list[position] !== undefined; moves++) {
    const pos = position + parseInt(list[position], 10);
    list[position] = (parseInt(list[position], 10) + 1).toString();
    position = pos;
  }
  console.log(moves);
};

go(list);
