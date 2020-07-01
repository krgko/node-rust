const ffi = require("ffi");
const path = require("path");

const libfizzbuzz = ffi.Library(
  path.join(__dirname, "../fizzbuzz/target/release/libfizzbuzz.so"),
  {
    fizzbuzz: ["string", ["int"]],
  }
);

const fizz_buzz = (n) => {
  let result = "";
  for (let i = 1; i <= n; i++) {
    let fizz_buzz_decision = "";
    if (i % 3 === 0) {
      fizz_buzz_decision += "fizz";
    }
    if (i % 5 === 0) {
      fizz_buzz_decision += "buzz";
    }

    if (fizz_buzz_decision === "") {
      fizz_buzz_decision = i.toString();
    }

    if (i !== 1) {
      result += "," + fizz_buzz_decision;
    } else {
      result += fizz_buzz_decision;
    }
  }
  return result;
};

console.time("node_run");
const node_run = fizz_buzz(100);
console.timeEnd("node_run");
console.time("rust_run");
const rust_run = libfizzbuzz.fizzbuzz(100);
console.timeEnd("rust_run");

console.log("\nSame result:", node_run === rust_run);
