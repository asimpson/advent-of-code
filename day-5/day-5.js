const fs = require('fs');

const start = () => {
  console.time('moves');
  fs.readFile(
    '/Users/asimpson/Projects/advent-of-code-2017/day-5/input.txt',
    (err, data) => {
      go(data.toString().trim().split('\n').map(x => parseInt(x, 10)));
    }
  );
};

const go = list => {
  var moves = 0;
  var position = 0;
  for (; list[position] !== undefined; moves++) {
    const pos = position + list[position];
    list[position] = list[position] >= 3
      ? list[position] - 1
      : list[position] + 1;
    position = pos;
  }
  console.log(moves);
  console.timeEnd('moves');
};

start();

module.exports = start;
