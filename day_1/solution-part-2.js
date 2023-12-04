const fs = require('fs');

const mappings = {
  "one": 1,
  "two": 2,
  "three": 3,
  "four": 4,
  "five": 5,
  "six": 6,
  "seven": 7,
  "eight": 8,
  "nine": 9,
};

const firstAndLastOfOccurrence = (entry, line) => ([
  Math.min(...entry.map(val => line.indexOf(val)).filter(i => i >= 0)),
  Math.max(...entry.map(val => line.lastIndexOf(val)).filter(i => i >= 0)),
  entry[1],
]);

const lineCode = line => {
  const positions = Object
    .entries(mappings)
    .map(entry => firstAndLastOfOccurrence(entry, line));

  const minPosition = positions.sort((a, b) => a[0] - b[0])[0];
  const maxPosition = positions.sort((a, b) => b[1] - a[1])[0];

  return parseInt(`${minPosition[2]}${maxPosition[2]}`);
}

const lines = fs.readFileSync('input.txt', {
  encoding: 'utf8',
  flag: 'r',
}).split('\n');
lines.pop();

console.log(
  lines.reduce((sum, line) => {
    sum = sum + lineCode(line)
    return sum;
  }, 0)
);
