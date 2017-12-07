const fs = require('fs');
let list;

fs.readFile('./input.txt', (err, data) => {
  list = data.toString().trim().split('\n').map(x => parseInt(x, 10));
  go(list);
});

const go = list => {
  var moves = 0;
  var position = 0;
  for (; list[position] !== undefined; moves++) {
    const pos = position + list[position];
    list[position] = list[position] + 1;
    position = pos;
  }
  console.log(moves);
};
