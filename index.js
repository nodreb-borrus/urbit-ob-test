const ob = require('urbit-ob');

function patpprint(num) {
  console.log(num, ob.patp(num));
}

function patqprint(num) {
  console.log(num, ob.patq(num).replace(/^~(doz)?/, '.~'));
}

// for (let i=0; i<=100; i++) {
//   const n = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
//   patpprint(n);
// }

const [, , operation, startStr, endStr] = process.argv;

if ( !startStr || !endStr || !operation ) {
  console.log("Invalid input. Please provide a valid operation and two numbers.");
  process.exit(1);
}

const start = parseInt(startStr);
const end = parseInt(endStr);

const operations = {
  'patp': patpprint,
  'patq': patqprint,
};

if (isNaN(start) || isNaN(end) || start > end) {
  console.log("Invalid range, invalid numbers.");
} else if (!operations[operation]) {
  console.log("Invalid operation, pick a real one: patp, patq.");
} else {
  for (let i = start; i <= end; i++) {
    operations[operation](i);
  }
}

