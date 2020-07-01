# Try node with rust lib

This project created to test about NodeJS and Rust by comparing their performance.

## Getting started

Build rust lib

On linux will got `libfizzbuzz.so`, macos will got `libfizzbuzz.dylib`. and lib should store at `/usr/lib` or `/usr/local/lib`

```bash
cd fizzbuzz && cargo build --release # lighter C binary file
```

Testing performance

```bash
cd fizzbuzz-compare && npm install && node app.js
```

Or easier steps

```bash
cd fizzbuzz-compare
npm run build_lib
npm start
```

## Performance testing

```bash
$ node app.js
node_run: 0.337ms
rust_run: 0.161ms

Same result: true
Done in 0.12s.
```

## Reference links

- [retrieve-raw-value-CString](https://doc.rust-lang.org/std/ffi/struct.CString.html#method.from_raw)
- [allocated-string](http://jakegoulding.com/rust-ffi-omnibus/string_return/)
- [nodeffi-tutorial](https://github.com/node-ffi/node-ffi/wiki/Node-FFI-Tutorial)
- [useful-clue](https://stackoverflow.com/questions/42496523/how-to-return-string-value-from-a-rust-ffi-function-in-nodejs)
